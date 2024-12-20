use std::{
    sync::{Arc, Mutex},
    net::{TcpStream, SocketAddr},
};

use hickory_resolver::{config::*, Resolver};
use url::Url;

use j4fsmb_parser::Command;

use crate::request::RequestBuilder;

#[derive(Clone)]
pub struct Client {
    client_socket: Arc<Mutex<TcpStream>>,
}

pub struct ClientBuilder {
    dns_resolver: Arc<Resolver>,
    url: Option<Url>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        let dns_resolver = Arc::new(
            Resolver::new(
                ResolverConfig::default(), 
                ResolverOpts::default()
            ).unwrap()
        );
        ClientBuilder {
            dns_resolver,
            url: None,
        }
    }

    pub fn url(mut self, url: Url) -> ClientBuilder {
        self.url = Some(url);
        self
    }

    pub async fn build(self) -> anyhow::Result<Client> {
        let url = self.url.ok_or(anyhow::anyhow!("No url provided"))?;
        let response = self.dns_resolver.lookup_ip(url.to_string())?;
        let addr = response.iter().next()
            .ok_or(anyhow::anyhow!("No addresses returned by dns resolver"))?;

        let socket_addr = SocketAddr::new(addr.into(), 445);
        let stream = TcpStream::connect(socket_addr)?;

        Ok(Client::new(stream))
    }
}

impl Client {
    pub fn new(tcp_stream: TcpStream) -> Self {
        let client_socket = Arc::new(Mutex::new(tcp_stream));
        Self {
            client_socket,
        }
    }

    pub fn read(&self) -> RequestBuilder {
        RequestBuilder::new(self.client_socket.clone())
            .command(Command::Read)
    }

    pub fn write(&self) -> RequestBuilder {
        RequestBuilder::new(self.client_socket.clone())
            .command(Command::Write)
    }

    pub fn create(&self) -> RequestBuilder {
        RequestBuilder::new(self.client_socket.clone())
            .command(Command::Create)
    }

    pub fn close(&self) -> RequestBuilder {
        RequestBuilder::new(self.client_socket.clone())
            .command(Command::Close)
    }
}

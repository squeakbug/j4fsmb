use std::net::{IpAddr, SocketAddr};

use tokio::{
    self,
    net::{TcpListener, TcpStream},
};
use tracing::{error, info, warn};

use crate::{
    config::ServerConfig,
    smb::{
        connection_manager::ConnectionManager,
        share::Share,
        transport_connection::TransportConnection,
    }
};

use super::gss_provider::GSSProvider;

pub struct Server {
    server_guid: String,
    addr: IpAddr,
    port: u16,
    shares: Vec<Share>,
    security_provider: GSSProvider,
    connection_manager: ConnectionManager,
}

impl Server {
    pub fn from_config(config: ServerConfig) -> anyhow::Result<Server> {
        let addr = config.addr.parse()
            .map_err(|err| anyhow::anyhow!("addr parse failed: {err:?}"))?;

        Ok(Self {
            server_guid: config.guid,
            addr,
            port: config.port,
            shares: Vec::new(),
            security_provider: GSSProvider::new(),
            connection_manager: ConnectionManager::new(),
        })
    }

    async fn handle_connect_request(
        &self, 
        socket: TcpStream
    ) -> anyhow::Result<()> {
        let conn = TransportConnection::from_socket(socket);
        tokio::spawn(async move {
            
        });
        Ok(())
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let socket_addr = SocketAddr::new(self.addr.into(), self.port);
        let listener = TcpListener::bind(socket_addr).await?;

        loop {
            match listener.accept().await {
                Ok((socket, _)) => self.handle_connect_request(socket).await?,
                Err(e) => warn!("Failed to accept connection: {}", e),
            }
        }
    }
}

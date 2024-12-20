use chrono::prelude::*;
use tokio::{
    self,
    net::TcpStream,
    sync::oneshot,
};

use super::request::Request;

pub struct TransportConnection {
    pub client_socket: TcpStream,
    pub buffer: Vec<u8>,
    pub send_queue: oneshot::Sender<Request>,

    pub created_at: DateTime<Utc>,
    pub last_received_at: DateTime<Utc>,
    pub last_send_at: DateTime<Utc>,
}

impl TransportConnection {
    pub fn from_socket(downstream: TcpStream) -> Self {
        let (tx, _rx) = oneshot::channel();
        TransportConnection {
            client_socket: downstream,
            buffer: Vec::new(),
            send_queue: tx,
            created_at: Utc::now(),
            last_received_at: Utc::now(),
            last_send_at: Utc::now(),
        }
    }

    pub async fn send() {

    }

    pub async fn receive() {

    }

    pub async fn close() {
        
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
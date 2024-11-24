use tokio::{
    self,
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt, AsyncWriteExt},
};
use log::{info, error};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let addr = "0.0.0.0:445";
    let listener = TcpListener::bind(addr).await?;
    info!("SMB server listening on {}", addr);

    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                info!("Accepted connection from {:?}", socket.peer_addr());
                tokio::spawn(handle_connection(socket));
            }
            Err(e) => error!("Failed to accept connection: {}", e),
        }
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buffer = [0u8; 1024];

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                info!("Connection closed by client");
                break;
            }
            Ok(n) => {
                let packet = &buffer[..n];
                log_packet(packet);

                if let Err(e) = socket.write_all(packet).await {
                    error!("Failed to write to socket: {}", e);
                    break;
                }
            }
            Err(e) => {
                error!("Failed to read from socket: {}", e);
                break;
            }
        }
    }
}

fn log_packet(packet: &[u8]) {
    let hex_string: String = packet.iter().map(|b| format!("{:02x}", b)).collect();
    info!("Received packet: {}", hex_string);
}

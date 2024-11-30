pub mod smb;
pub mod config;

use tracing::info;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();

    let config = config::load_config().expect("failed to load config: ");
    info!("{:?}", config);

    smb::server::Server::from_config(config.server)
        .expect("failed to load config: ")
        .run().await
        .expect("failed run server: ");

    Ok(())
}

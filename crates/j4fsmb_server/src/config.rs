use std::{
    env,
    fs::File,
    io::Read,
};

use serde_derive::Deserialize;
use toml;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub addr: String,
    pub guid: String,
    pub gss_provider: String,
    pub max_packet_size: usize,
}

pub fn load_config() -> anyhow::Result<Config> {
    let config_path = env::var("CONFIG_PATH")
        .map_err(|err| anyhow::anyhow!("env::var: {err:?}"))?;

    let mut file = File::open(config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    toml::de::from_str(&contents)
        .map_err(|err| anyhow::anyhow!("deserialize: {err:?}"))
}

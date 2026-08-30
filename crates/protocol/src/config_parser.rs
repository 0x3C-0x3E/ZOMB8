use std::{fs::File, io::Read, sync::OnceLock};

use serde::{Deserialize, Serialize};

pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server_ip: String,
    pub tps: u32,
    pub port: u16,
}

pub fn parse_config() -> anyhow::Result<()> {
    let mut file = File::open("assets/config.toml")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let config: Config = toml::from_str(&buf)?;
    CONFIG.set(config).unwrap();

    Ok(())
}

pub fn server_ip() -> String {
    CONFIG.get().unwrap().server_ip.clone()
}

pub fn tps() -> u32 {
    CONFIG.get().unwrap().tps
}

pub fn port() -> u16 {
    CONFIG.get().unwrap().port
}

use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server_ip: String,
}

pub fn parse_config() -> anyhow::Result<Config> {
    let mut file = File::open("assets/config.toml")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let config: Config = toml::from_str(&buf)?;

    Ok(config)
}

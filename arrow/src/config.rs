use std::{io::ErrorKind, ops::RangeInclusive};
use tokio::fs::{read_to_string, write};

use log::{error, info, warn};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    port: u16,
    host: String,
    motd: String,
    version_range: RangeInclusive<i32>,
}

impl Config {
    /// Get a reference to the config's port.
    pub fn port(&self) -> &u16 {
        &self.port
    }

    /// Get a reference to the config's host.
    pub fn host(&self) -> &String {
        &self.host
    }

    /// Get a reference to the config's motd.
    pub fn motd(&self) -> &String {
        &self.motd
    }

    /// Get a reference to the config's version range.
    pub fn version_range(&self) -> &RangeInclusive<i32> {
        &self.version_range
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 25565,
            host: "0.0.0.0".to_string(),
            motd: "Arrow - A minecraft server written in Rust".to_string(),
            version_range: 47..=754,
        }
    }
}

pub async fn load_config() -> Config {
    let config = match read_to_string("config.toml").await {
        Ok(s) => s,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            warn!("Config file not found. Creating one.");

            match write("config.toml", toml::to_string(&Config::default()).unwrap()).await {
                Ok(_) => info!("Config file created successfully."),
                Err(e) => error!("Failed creating config file: {}.", e),
            };

            return Config::default();
        },
        Err(e) => {
            error!("Failed reading config file: {}", e);
            return Config::default();
        }
    };

    match toml::from_str(&config) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed parsing config file: {}", e);
            Config::default()
        }
    }
}

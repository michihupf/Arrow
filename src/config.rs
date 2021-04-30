use std::io::ErrorKind;

use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use tokio::fs;
use toml::{from_str, to_string_pretty};

#[derive(Serialize, Deserialize)]
/// The configuration of the server
pub struct Config {
    /// The port the server binds to
    port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self { port: 25565 }
    }
}

impl Config {
    pub fn port(&self) -> u16 {
        self.port
    }
}

pub async fn read_config() -> Config {
    let string = match fs::read_to_string("config.toml").await {
        Ok(string) => string.clone(),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                warn!("Config file not found, creating one.");

                match fs::write(
                    "config.toml",
                    to_string_pretty(&Config::default()).unwrap().as_bytes(),
                )
                .await
                {
                    Ok(_) => info!("Sucessfully created config file"),
                    Err(e) => error!("Failed creating config file: {}", e),
                };

                return Config::default();
            }

            _ => {
                error!("Could not read config file: {} using default one", e);

                return Config::default();
            }
        },
    };

    match from_str(string.as_str()) {
        Ok(config) => config,
        Err(e) => {
            error!("failed to parse config: {} using default one", e);
            Config::default()
        }
    }
}

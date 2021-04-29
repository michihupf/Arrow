use std::io::ErrorKind;

use log::{info, warn, error};
use tokio::fs;
use toml::{from_str, to_string_pretty};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 25565,
        }
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

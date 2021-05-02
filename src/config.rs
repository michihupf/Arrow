use std::{
    io::ErrorKind,
    ops::RangeInclusive,
};

use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use tokio::fs;
use toml::{from_str, to_string_pretty};

#[derive(Serialize, Deserialize)]
/// The configuration of the server
pub struct Config {
    /// The port the server binds to
    port: u16,
    /// The protocol version (range)
    ///
    /// The smallest number is beeing used for status
    protocol_version_start: i32,
    protocol_version_end: i32,
    /// The name given when getting status (e.g. 1.16.x)
    version_name: String,
    /// The MOTD
    description: Description,
}

#[derive(Serialize, Deserialize)]
pub struct Description {
    pub extra: Vec<Extra>,
    /// Write some text here
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Extra {
    /// e.g. `yellow`
    color: Option<String>,
    /// e.g. `foo`
    text: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 25565,
            protocol_version_start: 47,
            protocol_version_end: 47,
            version_name: "Arrow 1.16.x".to_string(),
            description: Description::default(),
        }
    }
}

impl Default for Description {
    fn default() -> Self {
        Self {
            extra: vec![],
            text: "Arrow - A Minecraft Server written in Rust".to_string(),
        }
    }
}

impl Config {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn protocol_version(&self) -> RangeInclusive<i32> {
        self.protocol_version_start..=self.protocol_version_end
    }

    pub fn version_name(&self) -> &String {
        &self.version_name
    }

    pub fn description(&self) -> &Description {
        &self.description
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

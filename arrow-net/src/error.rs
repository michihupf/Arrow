use std::{error::Error, fmt::Display};

pub type Result<R> = std::result::Result<R, NetError>;

#[derive(Debug)]
pub enum NetError {
    ServerBindError(String),
    ClientAcceptError(String),
}

impl Display for NetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ServerBindError(m) => write!(f, "Failed binding: {}", m),
            Self::ClientAcceptError(m) => write!(f, "Failed accepting client connection: {}", m),
        }
    }
}

impl Error for NetError {}

use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum NetError {
    /// produced when serde failed to deserialize a packet
    DeserializeError(String),
    SerializeError(String),

    /// produced when packet data is invalid
    InvalidPacketDataError(String),
    
    /// produced when failing to read data
    ReadError(String),
    /// produced when failing to send data
    WriteError(String),
}

impl Error for NetError { }

impl Display for NetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeserializeError(s) => write!(f, "Deserialize Error: {}", s),
            Self::SerializeError(s) => write!(f, "Serialize Error: {}", s),
            Self::InvalidPacketDataError(s) => write!(f, "Invalid Packet Error: {}", s),
            Self::ReadError(s) => write!(f, "Failed Reading: {}", s),
            Self::WriteError(s) => write!(f, "Failed Writing: {}", s),
        }
    }
}

use std::{error::Error, fmt::Display};

use arrow_codec::error::DecoderError;

pub type Result<R> = std::result::Result<R, NetError>;

#[derive(Debug)]
pub enum NetError {
    ServerBindError(String),
    ClientAcceptError(String),
    DecoderError(String),
    InvalidStatus(i32),
    UnexpectedEof,
    UnexpectedPacket,
}

impl Display for NetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ServerBindError(m) => write!(f, "Failed binding: {}", m),
            Self::ClientAcceptError(m) => write!(f, "Failed accepting client connection: {}", m),
            Self::DecoderError(m) => write!(f, "Failed decoding: {}", m),
            Self::InvalidStatus(status) => write!(f, "Invalid status {}", status),
            Self::UnexpectedEof => write!(f, "Unexpected eof"),
            Self::UnexpectedPacket => write!(f, "Unexpected packet"),
        }
    }
}

impl From<DecoderError> for NetError {
    fn from(e: DecoderError) -> Self {
        Self::DecoderError(e.0)
    }
}

impl Error for NetError {}

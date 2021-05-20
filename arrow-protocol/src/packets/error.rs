use std::{error::Error, fmt::Display};

use crate::serde::error::SerdeError;

use super::State;

#[derive(Debug)]
pub enum PacketError {
    SerdeError(String),
    InvalidPacketId(i32, State),
}

impl From<SerdeError> for PacketError {
    fn from(e: SerdeError) -> Self {
        Self::SerdeError(format!("{}", e))
    }
}

impl Error for PacketError {}

impl Display for PacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerdeError(m) => write!(f, "{}", m),
            Self::InvalidPacketId(id, state) => {
                write!(f, "Invalid id {:02x} in state {:?}", id, state)
            }
        }
    }
}

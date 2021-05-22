use std::{error::Error, fmt::Display};

use crate::serde::error::SerdeError;

use super::State;

/// The error for packets.
#[derive(Debug)]
pub enum PacketError {
    /// Returned when serializing or deserializing failed.
    SerdeError(String),
    /// Returned when the id is unknown for the current status.
    InvalidPacketId(i32, State),
    /// Returned when converting to a json string fails
    BuildingJsonFailed,
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
            Self::BuildingJsonFailed => {
                write!(f, "Building json string failed")
            }
        }
    }
}

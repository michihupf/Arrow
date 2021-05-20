use std::io::Error;

use arrow_protocol::packets::error::PacketError;

/// A error for the encoder.
#[derive(Debug)]
pub struct EncoderError(pub String);

/// A error for the decoder.
#[derive(Debug)]
pub struct DecoderError(pub String);

impl From<Error> for EncoderError {
    fn from(e: Error) -> Self {
        Self(format!("Failed encoding: {}", e))
    }
}

impl From<PacketError> for EncoderError {
    fn from(e: PacketError) -> Self {
        Self(format!("Failed encoding packet: {}", e))
    }
}

impl From<Error> for DecoderError {
    fn from(e: Error) -> Self {
        Self(format!("Failed decoding: {}", e))
    }
}

impl From<PacketError> for DecoderError {
    fn from(e: PacketError) -> Self {
        Self(format!("Failed decoding packet: {}", e))
    }
}

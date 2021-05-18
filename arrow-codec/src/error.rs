use std::io::Error;

pub struct EncoderError(pub String);
pub struct DecoderError(pub String);

impl From<Error> for EncoderError {
    fn from(e: Error) -> Self {
        Self(format!("Failed encoding: {}", e))
    }
}

impl From<Error> for DecoderError {
    fn from(e: Error) -> Self {
        Self(format!("Failed decoding: {}", e))
    }
}


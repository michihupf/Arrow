use std::io::Error;

#[derive(Debug)]
pub struct EncoderError(pub String);
#[derive(Debug)]
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

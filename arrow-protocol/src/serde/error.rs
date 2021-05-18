pub type Result<R> = std::result::Result<R, SerdeError>;

#[derive(Debug, PartialEq, Eq)]
pub enum SerdeError {
    UnexpectedEof,
    SerializeError(String),
    DeserializeError(String),
}

impl std::error::Error for SerdeError {}

impl serde::de::Error for SerdeError {
    fn custom<T>(t: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::DeserializeError(format!("{}", t))
    }
}

impl serde::ser::Error for SerdeError {
    fn custom<T>(t: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::SerializeError(format!("{}", t))
    }
}

impl std::fmt::Display for SerdeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEof => write!(f, "Unexpected eof"),
            Self::SerializeError(e) => write!(f, "Failed serializing: {}", e),
            Self::DeserializeError(e) => write!(f, "Failed deserializing: {}", e),
        }
    }
}

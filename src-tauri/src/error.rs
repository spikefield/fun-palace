use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct TwelvetyError {
    pub code: String,
    pub message: String,
}

impl fmt::Display for TwelvetyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for TwelvetyError {}

impl From<std::io::Error> for TwelvetyError {
    fn from(err: std::io::Error) -> Self {
        TwelvetyError {
            code: "IO_ERROR".to_string(),
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for TwelvetyError {
    fn from(err: serde_json::Error) -> Self {
        TwelvetyError {
            code: "JSON_ERROR".to_string(),
            message: err.to_string(),
        }
    }
}

use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct FunPalaceError {
    pub code: String,
    pub message: String,
}

impl fmt::Display for FunPalaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for FunPalaceError {}

impl From<std::io::Error> for FunPalaceError {
    fn from(err: std::io::Error) -> Self {
        FunPalaceError {
            code: "IO_ERROR".to_string(),
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for FunPalaceError {
    fn from(err: serde_json::Error) -> Self {
        FunPalaceError {
            code: "JSON_ERROR".to_string(),
            message: err.to_string(),
        }
    }
}

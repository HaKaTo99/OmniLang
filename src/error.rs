use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum OmniError {
    Io(String),
    Json(String),
    Network(String),
    InvalidInput(String),
    Unsupported(String),
}

impl fmt::Display for OmniError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OmniError::Io(msg) => write!(f, "IO error: {}", msg),
            OmniError::Json(msg) => write!(f, "JSON error: {}", msg),
            OmniError::Network(msg) => write!(f, "Network error: {}", msg),
            OmniError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            OmniError::Unsupported(msg) => write!(f, "Unsupported: {}", msg),
        }
    }
}

impl std::error::Error for OmniError {}

impl From<std::io::Error> for OmniError {
    fn from(err: std::io::Error) -> Self {
        OmniError::Io(err.to_string())
    }
}

impl From<serde_json::Error> for OmniError {
    fn from(err: serde_json::Error) -> Self {
        OmniError::Json(err.to_string())
    }
}

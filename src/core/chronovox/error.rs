use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChronovoxError {
    #[error("Storage error: {0}")]
    Storage(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Missing field: {0}")]
    MissingField(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid event kind: {0}")]
    InvalidEventKind(String),
}

pub type Result<T> = std::result::Result<T, ChronovoxError>;

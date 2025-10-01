// supabasic/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SupabasicError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, SupabasicError>;

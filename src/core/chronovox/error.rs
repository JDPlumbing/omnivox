use thiserror::Error;
use crate::supabasic::error::SupabasicError;

#[derive(Debug, Error)]
pub enum ChronovoxError {
    #[error("Supabase error: {0}")]
    Db(String),

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

// unify everything under Chronovox
impl From<SupabasicError> for ChronovoxError {
    fn from(err: SupabasicError) -> Self {
        ChronovoxError::Db(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ChronovoxError>;

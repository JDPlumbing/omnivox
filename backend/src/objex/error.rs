use thiserror::Error;
use crate::supabasic;

#[derive(Debug, Error)]
pub enum ObjexError {
    #[error("Supabase error: {0}")]
    Supabase(#[from] supabasic::SupabasicError),

    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("UUID parse error: {0}")]
    Uuid(#[from] uuid::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Shorthand result type for Objex
pub type Result<T> = std::result::Result<T, ObjexError>;

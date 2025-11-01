use thiserror::Error;
use crate::objex;
use crate::chronovox;
use crate::supabasic;


#[derive(Debug, Error)]
pub enum OmnivoxError {
    #[error(transparent)]
    Supa(#[from] supabasic::SupabasicError),

    #[error(transparent)]
    Chrono(#[from] chronovox::ChronovoxError),

    #[error(transparent)]
    Objex(#[from] objex::ObjexError),

    #[error(transparent)]
    Uuid(#[from] uuid::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    ChronoParse(#[from] chrono::ParseError),  // 👈 added this

    #[error("Invalid simulation row: {0}")]
    InvalidRow(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Simulation load error: {0}")]
    LoadError(String),

    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, OmnivoxError>;

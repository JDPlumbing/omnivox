use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateNoteDto {
    /// Optional: attach note to existing entity
    pub entity_id: Option<Uuid>,

    pub text: String,
}

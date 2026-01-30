use serde::{Serialize, Deserialize};
use crate::core::id::EntityId;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
}

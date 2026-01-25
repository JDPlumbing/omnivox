use serde::{Serialize, Deserialize};
use crate::core::EntityId;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
}

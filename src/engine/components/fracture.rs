use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::core::id::entity_id::EntityId;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct FractureData {
    pub entity_id: EntityId,
    pub plane: String,
    pub energy: f64,
    pub threshold: f32,
}

use serde::{Serialize, Deserialize};
use crate::core::id::entity_id::EntityId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationData {
    pub entity_id: EntityId,
    pub corrosion: f64,
    pub fatigue: f64,
    pub thermal: f64,
    pub total_integrity: f64,
}

use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::core::id::entity_id::EntityId;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct CorrosionData {
    pub entity_id: EntityId,
    pub surface_area: f64,     // m², to localize corrosion loss
    pub thickness_loss: f64,   // m, material lost over time
    pub rate: f64,             // m/s (corrosion rate)
    pub environment_factor: f32, // humidity, salinity, acidity, etc.
    pub severity: f64,         // 0.0–1.0, normalized damage
}

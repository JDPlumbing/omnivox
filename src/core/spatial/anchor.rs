use crate::core::WorldId;
use crate::core::UvoxId;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialAnchor {
    pub world_id: WorldId,
    pub uvox: UvoxId,
    pub address_id: Option<Uuid>,
}

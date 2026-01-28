use crate::core::WorldId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WorldMembership {
    pub world_id: WorldId,
}

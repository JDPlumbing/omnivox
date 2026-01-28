use serde::{Serialize, Deserialize};
use crate::core::tdt::sim_time::SimTime;
use crate::core::id::WorldId;



/// -------------------------------------------------------------------
/// Domain-level metadata about a world (NOT persisted directly)
/// -------------------------------------------------------------------
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct World {
    pub id: WorldId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub world_epoch: Option<SimTime>,
}

impl World {
    pub fn new(id: WorldId, name: Option<String>, description: Option<String>, epoch: Option<SimTime>) -> Self {
        Self {
            id,
            name,
            description,
            world_epoch: epoch,
        }
    }
}


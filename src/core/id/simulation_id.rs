use serde::{Serialize, Deserialize};

use super::world_id::WorldId;
use super::user_id::UserId;
use super::uvox_region_id::UvoxRegionId;
use crate::core::tdt::SimTime; // assuming this exists

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SimulationId {
    pub world: WorldId,
    pub region: UvoxRegionId,
    pub time_start: SimTime,
    pub user: UserId,
    pub branch: u32, // allows forks / variations
}

impl SimulationId {
    pub fn new(
        world: WorldId,
        region: UvoxRegionId,
        time_start: SimTime,
        user: UserId,
        branch: u32,
    ) -> Self {
        Self { world, region, time_start, user, branch }
    }
}

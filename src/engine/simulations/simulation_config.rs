use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::core::id::{WorldId, UserId};
use crate::core::id::SimulationId;
use crate::core::tdt::sim_time::SimTime;
use crate::core::UvoxRegionId;

/// Full configuration required to start a simulation instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// Which world (Earth, cloned worlds, Mars, custom, etc.)
    pub world_id: WorldId,

    /// Spatial bounds for the simulation (user property, region of interest, etc.)
    pub region: UvoxRegionId,

    /// Simulation starting time (often derived from "year built" of house or now())
    pub start_time: SimTime,

    /// The user who owns this simulation
    pub user_id: UserId,

    /// Optional branch index for alternate timelines
    pub branch: u32,

    /// Extra metadata (user’s address, notes, tags, source info, etc.)
    pub metadata: Value,
}
impl SimulationConfig {
    /// Most common path: Earth world, region around a center point, starting now, user-owned.
    pub fn basic(world_id: WorldId, region: UvoxRegionId, user_id: UserId) -> Self {
        Self {
            world_id,
            region,
            start_time: SimTime::now(),
            user_id,
            branch: 0,
            metadata: serde_json::json!({}),
        }
    }

    /// Full constructor for total control.
    pub fn new(
        world_id: WorldId,
        region: UvoxRegionId,
        start_time: SimTime,
        user_id: UserId,
        branch: u32,
        metadata: Value,
    ) -> Self {
        Self {
            world_id,
            region,
            start_time,
            user_id,
            branch,
            metadata,
        }
    }
}

impl SimulationConfig {
    /// Generate a SimulationId from this config.
    pub fn to_simulation_id(&self) -> SimulationId {
        SimulationId::new(
            self.world_id,
            self.region,
            self.start_time,
            self.user_id,
            self.branch,
        )
    }
}
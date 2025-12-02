use serde::{Serialize, Deserialize};
use crate::core::id::{WorldId, UserId, UvoxRegionId};
use crate::core::uvoxid::UvoxId;
use crate::core::tdt::sim_time::SimTime;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SimulationId {
    pub world: WorldId,
    pub region: UvoxRegionId,
    pub time_start: SimTime,
    pub user: UserId,
    pub branch: u32,
}

impl SimulationId {
    #[inline]
    pub fn new(
        world: WorldId,
        region: UvoxRegionId,
        time_start: SimTime,
        user: UserId,
        branch: u32,
    ) -> Self {
        Self {
            world,
            region,
            time_start,
            user,
            branch,
        }
    }
}

impl fmt::Display for SimulationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// ------------------------------------------------------------
// Default impl
// ------------------------------------------------------------ 
impl Default for SimulationId {
    fn default() -> Self {
        SimulationId {
            world: WorldId::from(0),
            region: UvoxRegionId::default(),
            time_start: SimTime::from_ns(0),
            user: UserId::from(0),
            branch: 0,
        }
    }
}
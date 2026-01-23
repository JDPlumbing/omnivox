use serde::{Serialize, Deserialize};
use crate::core::id::{WorldId, UserId, UvoxRegionId};
//use crate::core::uvoxid::UvoxId;
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

// ------------------------------------------------------------
// Default impl
// ------------------------------------------------------------ 
impl Default for SimulationId {
    fn default() -> Self {
        SimulationId {
            world: WorldId::from(0),
            region: UvoxRegionId::default(),
            time_start: SimTime::from_ns(0),
            user: UserId::zero(),
            branch: 0,
        }
    }
}

use uuid::Uuid;


impl TryFrom<Uuid> for SimulationId {
    type Error = anyhow::Error;

    fn try_from(_u: Uuid) -> Result<Self, Self::Error> {
        Err(anyhow!(
            "SimulationId is structured and cannot be built from a UUID"
        ))
    }
}

use std::str::FromStr;
use anyhow::{anyhow};

// ------------------------------------------------------------
// Display: WORLD-REGION-STARTNS-USER-BRANCH
// ------------------------------------------------------------
impl fmt::Display for SimulationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}


// ------------------------------------------------------------
// Parse SimulationId from string
// ------------------------------------------------------------
impl FromStr for SimulationId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}
impl SimulationId {
    /// Convert the structured ID into a hashed, opaque API ID string.
    /// Uses SipHash (Rust's default hasher) — fast and stable.
    pub fn to_api_id(&self) -> String {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash = hasher.finish();

        format!("{:016x}", hash)
    }
}

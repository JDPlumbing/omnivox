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
use anyhow::{anyhow, Error};

// ------------------------------------------------------------
// Display: WORLD-REGION-STARTNS-USER-BRANCH
// ------------------------------------------------------------
impl fmt::Display for SimulationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}-{}-{}",
            self.world.0,
            self.region.to_compact_string(),
            self.time_start.0,
            self.user.0,
            self.branch
        )
    }
}


// ------------------------------------------------------------
// Parse SimulationId from string
// ------------------------------------------------------------
impl FromStr for SimulationId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();

        if parts.len() != 5 {
            return Err(anyhow!("Invalid SimulationId '{}'", s));
        }

        // Parse each component
        let world    = WorldId(parts[0].parse()?);
        let region   = UvoxRegionId::from_compact(parts[1])?;
        let start    = SimTime(parts[2].parse()?);
        let user     = UserId(parts[3].parse()?);
        let branch   = parts[4].parse::<u32>()?;

        Ok(SimulationId {
            world,
            region,
            time_start: start,
            user,
            branch,
        })
    }
}

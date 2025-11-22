
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::collections::HashMap;

use crate::supabasic::events::EventRow;
use crate::supabasic::worlds::WorldRecord;

use crate::sim::clock::SimClock;
use crate::sim::components::SimComponents;
use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_duration::SimDuration;

use crate::sim::entity::SimEntity;

/// -------------------------------------------------------------------
/// Domain-level metadata about a world (NOT persisted directly)
/// -------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub world_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub world_epoch: Option<SimTime>,
}

impl World {
    pub fn new(world_id: i64, name: Option<String>, description: Option<String>, epoch: Option<SimTime>) -> Self {
        Self {
            world_id,
            name,
            description,
            world_epoch: epoch,
        }
    }
}

impl Default for World {
    fn default() -> Self {
        World {
            world_id: 0,
            name: Some("Test-Earth".into()),
            description: None,
            world_epoch: Some(SimTime::from_ns(0)),
        }
    }
}


/// -------------------------------------------------------------------
/// In-memory simulation state for a running world
/// -------------------------------------------------------------------
#[derive(Debug)]
pub struct WorldState {
    pub meta: World,                      // ✔ use domain model
    pub entities: HashMap<Uuid, SimEntity>,
    pub events: Vec<EventRow>,

    pub sim_time: SimTime,
    pub sim_delta: SimDuration,
    pub clock: Option<SimClock>,

    pub components: SimComponents,
}

impl WorldState {
    pub fn new(meta: World) -> Self {
        Self {
            meta,
            entities: HashMap::new(),
            events: Vec::new(),
            sim_time: SimTime::from_ns(0),
            sim_delta: SimDuration::from_ns(0),
            clock: None,
            components: SimComponents::new(),
        }
    }

    pub fn from_entities(meta: World, entities: Vec<SimEntity>) -> Self {
        let mut map = HashMap::new();
        for ent in entities {
            map.insert(ent.entity_id, ent);
        }

        Self {
            meta,
            entities: map,
            events: Vec::new(),
            sim_time: SimTime::from_ns(0),
            sim_delta: SimDuration::from_ns(0),
            clock: None,
            components: SimComponents::new(),
        }
    }
}

impl Default for WorldState {
    fn default() -> Self {
        WorldState::new(World::default())
    }
}


impl From<WorldRecord> for World {
    fn from(rec: WorldRecord) -> Self {
        World {
            world_id: rec.world_id,
            name: rec.name,
            description: rec.description,
            world_epoch: rec.world_epoch.map(SimTime::from_ns),
        }
    }
}
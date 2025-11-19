use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::collections::HashMap;

use crate::supabasic::events::EventRow;
use crate::supabasic::WorldRecord;

use crate::sim::clock::SimClock;
use crate::sim::components::SimComponents;
use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_duration::SimDuration;

/// -------------------------------------------------------------------
/// Persistent metadata about a world (Supabase-side)
/// -------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    pub frame_id: i64,

    pub name: Option<String>,
    pub description: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    /// The simulation time that corresponds to the world’s “start”
    pub world_epoch: Option<SimTime>,

    /// Runtime-only (not persisted)
    #[serde(skip)]
    pub events: Vec<EventRow>,

    /// Runtime-only (not persisted)
    #[serde(skip)]
    pub entities: HashMap<Uuid, crate::sim::entity::SimEntity>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            frame_id: 0,
            name: Some("Test-Earth".into()),
            description: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            world_epoch: None,
            events: Vec::new(),
            entities: HashMap::new(),
        }
    }
}

/// -------------------------------------------------------------------
/// In-memory simulation state for a running world
/// -------------------------------------------------------------------
#[derive(Debug)]
pub struct WorldState {
    /// metadata row loaded from DB
    pub meta: WorldRecord,

    /// simulation objects already instantiated
    pub entities: HashMap<Uuid, crate::sim::entity::SimEntity>,

    /// raw event history (optional)
    pub events: Vec<EventRow>,

    /// absolute simulation time
    pub sim_time: SimTime,

    /// delta since last tick
    pub sim_delta: SimDuration,

    /// autonomous clock driving the sim (optional)
    pub clock: Option<SimClock>,

    /// ECS-style component storage
    pub components: SimComponents,
}

impl WorldState {
    pub fn new(meta: WorldRecord) -> Self {
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

    /// Construct a world from already-loaded entities
    pub fn from_entities(meta: WorldRecord, entities: Vec<crate::sim::entities::entity::SimEntity>) -> Self {
        let mut map = HashMap::new();
        for ent in entities {
            map.insert(ent.id, ent);
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
        Self::new(WorldRecord {
            frame_id: 0,
            name: Some("Default World".into()),
            description: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        })
    }
}

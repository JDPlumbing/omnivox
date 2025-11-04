use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::supabasic::events::EventRow;
use crate::supabasic::WorldRow;
use std::collections::HashMap;
use crate::objex::core::types::Objex;
use uuid::Uuid;
use crate::sim::components::{Velocity, Acceleration};   

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    pub frame_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    /// Runtime-only: events that occurred within this world
    #[serde(default)]
    pub events: Vec<EventRow>, // not persisted directly, populated by querying events
    /// Runtime-only: active objects within this world
    #[serde(skip)]
    pub objects: HashMap<String, Objex>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewWorld {
    pub frame_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            // whatever fields you have — just make a minimal stub
            frame_id: 0,
            name: Some("Test-Earth".into()),
            description: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            events: vec![],
            objects: HashMap::new(),

        }
    }
}

#[derive(Debug)]
pub struct WorldState {
    pub meta: WorldRow,                // persisted metadata
    pub events: Vec<EventRow>,         // runtime events
    pub objects: HashMap<String, Objex>, // runtime objects
    pub velocity_components: HashMap<Uuid, Velocity>,
    pub acceleration_components: HashMap<Uuid, Acceleration>,
}

impl WorldState {
    pub fn new(meta: WorldRow) -> Self {
        Self {
            meta,
            events: Vec::new(),
            objects: HashMap::new(),
            velocity_components: HashMap::new(),
            acceleration_components: HashMap::new(),
        }
    }
}
impl Default for WorldState {
    fn default() -> Self {
        Self::new(WorldRow {
            frame_id: 0,
            name: Some("Default World".into()),
            description: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        })
    }
}

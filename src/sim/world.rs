use uuid::Uuid;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::chronovox::{Timeline, UvoxId};
use crate::objex::Objex;
use crate::tdt::core::TimeDelta;

/// Pure state of a simulation world
pub struct SimWorld {
    pub simulation_id: Uuid,
    pub frame_id: u64,
    pub tick_rate: TimeDelta,
    pub last_saved: Option<DateTime<Utc>>,
    pub owner_id: Uuid,
    pub objects: HashMap<UvoxId, Objex>,
    pub timeline: Timeline,
    pub current_tick: i64,
    pub persist_events: bool,
}

impl Default for SimWorld {
    fn default() -> Self {
        Self {
            simulation_id: Uuid::parse_str("b691967d-8820-4f81-ab32-a9e7a10189f7").unwrap(),
            frame_id: 0,
            tick_rate: TimeDelta::from_ticks(1, "nanoseconds"),
            last_saved: None,
            owner_id: Uuid::parse_str("4ea96b3f-51d7-4238-bd18-2f7fd8be26ec").unwrap(),
            objects: HashMap::new(),
            timeline: Timeline::new(),
            current_tick: 0,
            persist_events: false,
        }
    }
}

/// Data Transfer Object for API responses
#[derive(Serialize)]
pub struct SimWorldDto {
    pub simulation_id: Uuid,
    pub frame_id: u64,
    pub tick_rate_ns: i64,
    pub last_saved: Option<DateTime<Utc>>,
    pub owner_id: Uuid,
    pub current_tick: i64,
    pub persist_events: bool,
    // TODO: objects and timeline → flatten into Vecs if needed
}

impl From<&SimWorld> for SimWorldDto {
    fn from(world: &SimWorld) -> Self {
        Self {
            simulation_id: world.simulation_id,
            frame_id: world.frame_id,
            tick_rate_ns: world.tick_rate.ticks("nanoseconds"),

            last_saved: world.last_saved,
            owner_id: world.owner_id,
            current_tick: world.current_tick,
            persist_events: world.persist_events,
        }
    }
}

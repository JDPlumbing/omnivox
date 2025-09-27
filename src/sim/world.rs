use uuid::Uuid;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use objex::Objex;
use chronovox::{Timeline, UvoxId};
use tdt::core::TimeDelta;

/// In-memory representation of a simulation.
pub struct SimWorld {
    /// Simulation container ID (primary key in `simulations` table)
    pub simulation_id: Uuid,

    /// Frame of reference for UvoxId coordinates (0 = canonical Earth, etc.)
    pub frame_id: u64,

    /// How much time one tick represents (ns per tick or similar)
    pub tick_rate: TimeDelta,

    /// Last time this simulation was saved
    pub last_saved: Option<DateTime<Utc>>,

    /// Owner of this simulation (foreign key to `auth.users` table)
    pub owner_id: Uuid,

    /// The entities that exist in this world, keyed by their spatial UvoxId
    pub objects: HashMap<UvoxId, Objex>,

    /// Chronological record of events in this simulation
    pub timeline: Timeline,
}

impl Default for SimWorld {
    fn default() -> Self {
        Self {
            simulation_id: Uuid::parse_str("b691967d-8820-4f81-ab32-a9e7a10189f7")
                .expect("hardcoded UUID should parse"),
            frame_id: 0, // test world always 0
            tick_rate: TimeDelta::from_ticks(1, "nanoseconds"),
            last_saved: None,
            owner_id: Uuid::parse_str("4ea96b3f-51d7-4238-bd18-2f7fd8be26ec")
                .expect("hardcoded UUID should parse"),
            objects: HashMap::new(),
            timeline: Timeline::new(),
        }
    }
}

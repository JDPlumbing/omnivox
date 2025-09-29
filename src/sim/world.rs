use uuid::Uuid;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::geospec::shapes::Sphere;
use crate::objex::{Objex, Shape, MaterialLink};
use crate::chronovox::{Timeline, ChronoEvent, EventKind, UvoxId};
use crate::tdt::core::TimeDelta;

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

    /// Current tick position of this world
    pub current_tick: i64,
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
            current_tick: 0,
        }
    }
}

impl SimWorld {
    /// Advance the simulation by one tick and apply events for that tick.
    pub fn tick(&mut self) -> Vec<ChronoEvent> {
        self.current_tick += 1;
        let now_tick = self.current_tick;

        // Collect events scheduled for this tick
        let ready: Vec<ChronoEvent> = self.timeline
            .events
            .iter()
            .filter(|ev| ev.t.ticks("nanoseconds") == now_tick)
            .cloned()
            .collect();

        // Apply the events to world state
        for ev in &ready {
            match ev.kind {
                EventKind::Spawn => {
                    // In a real impl, you’d construct the Objex from payload/DB
                    let obj = Objex {
                        entity_id: Uuid::new_v4(),
                        name: "test_obj".into(),
                        shape: Shape::Sphere(Sphere { radius: 1.0 }),

                        material: MaterialLink::vacuum(),
                    };

                    self.objects.insert(ev.id.clone(), obj);
                }
                EventKind::Despawn => {
                    self.objects.remove(&ev.id);
                }
                _ => {
                    // other event kinds can be added here
                }
            }
        }

        ready
    }
}

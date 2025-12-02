use crate::core::{
    chronovox::{ChronoEvent, EventKind},
};
use crate::sim::{
    systems::System,
    world::WorldState,
    components::acceleration::Acceleration,
};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GravitySystem;

impl System for GravitySystem {
    fn name(&self) -> &'static str { "GravitySystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        const EARTH_GRAVITY: f64 = -9.81;

        // If the clock isn't set, nothing happens
        let Some(clock) = &world.clock else {
            return events;
        };

        for (id, entity) in world.entities.iter() {
            //
            // Apply gravity to radial acceleration
            //
            let accel = world.components.acceleration_components
                .entry(*id)
                .and_modify(|a| a.ar += EARTH_GRAVITY)
                .or_insert(Acceleration {
                    ar: EARTH_GRAVITY,
                    alat: 0.0,
                    alon: 0.0,
                });

            //
            // Emit an acceleration event
            //
            events.push(
                ChronoEvent::new(
                    entity.id,    // correct new field
                    entity.world_id,     // required for ChronoEvent
                    clock.current,       // timestamp
                    EventKind::Accelerate {
                        ar: accel.ar,
                        alat: accel.alat,
                        alon: accel.alon,
                    }
                )
            );
        }

        events
    }
}

use crate::core::{
    chronovox::{ChronoEvent, EventKind},
    tdt::{sim_time::SimTime, sim_duration::SimDuration},
};

use crate::engine::{
    systems::System,
    world::WorldState,
};

use serde::{Serialize, Deserialize};
use crate::core::id::EntityId;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AccelerationSystem;

impl System for AccelerationSystem {
    fn name(&self) -> &'static str {
        "AccelerationSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        // Simulation time data
        let now = world.sim_time;
        let dt  = world.sim_delta;
        let end = now.add(dt);

        for (id, accel) in world.components.acceleration_components.iter() {
            // Must have velocity component to apply acceleration
            if let Some(velocity) = world.components.velocity_components.get_mut(id) {

                //
                // --- Apply acceleration to velocity ---
                //
                velocity.dr   += accel.ar;
                velocity.dlat += accel.alat;
                velocity.dlon += accel.alon;

                //
                // --- Correct SimEntity lookup ---
                //
                let Some(entity) = world.entities.get(&id) else {
                    continue;
                };

                //
                // --- Emit valid ChronoEvent ---
                //
                events.push(
                    ChronoEvent::new(
                        entity.id,   // UUID
                        entity.world_id,    // world ID
                        end,                // timestamp
                        EventKind::Accelerate {
                            ar: accel.ar,
                            alat: accel.alat,
                            alon: accel.alon,
                        }
                    )
                );
            }
        }

        events
    }
}

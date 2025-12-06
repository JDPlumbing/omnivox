use crate::sim::systems::System;
use crate::sim::world::WorldState;
use crate::core::chronovox::{ChronoEvent, EventKind};

use crate::core::uvoxid::RUm;

#[derive(Default, Debug)]
pub struct GravitySystem;

// Physical constants
const G: f64 = 6.67430e-11;            // m^3 kg^-1 s^-2
const EARTH_MASS: f64 = 5.9722e24;     // kg

impl System for GravitySystem {
    fn name(&self) -> &'static str { "GravitySystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = vec![];

        // Loop through every entity that has acceleration
        for (id, accel) in world.components.acceleration_components.iter_mut() {
            let entity = match world.entities.get(id) {
                Some(e) => e,
                None => continue,
            };

            // Convert r_um → meters
            let r_um = entity.position.r_um.0;
            let r_m = (r_um as f64) * 1e-6;

            // Prevent divide-by-zero
            if r_m <= 1.0 {
                accel.ar = 0.0;
                continue;
            }

            // Newtonian gravitational acceleration
            let a_r = G * EARTH_MASS / (r_m * r_m);

            // Radial acceleration is inward → negative
            accel.ar = -a_r;

            // Angular components unchanged
            accel.alat = 0.0;
            accel.alon = 0.0;

            // Emit event for debugging
            events.push(
                ChronoEvent::new(
                    entity.id,
                    entity.world_id,
                    world.sim_time,
                    EventKind::Accelerate {
                        ar: accel.ar,
                        alat: 0.0,
                        alon: 0.0,
                    }
                )
            );
        }

        events
    }
}

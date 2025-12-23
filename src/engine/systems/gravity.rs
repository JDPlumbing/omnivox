use crate::engine::systems::System;
use crate::engine::world::WorldState;
use crate::core::chronovox::ChronoEvent;
use crate::core::uvoxid::UvoxId;

use crate::core::env::gravity::total_gravity;

#[derive(Default, Debug)]
pub struct GravitySystem;

impl System for GravitySystem {
    fn name(&self) -> &'static str { "GravitySystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = vec![];

        let dt_s = world.sim_delta.as_secs_f64();

        for (id, entity) in world.entities.iter_mut() {
            if let Some(acc) = world.components.acceleration_components.get_mut(id) {

                // Compute physical gravity at this UvoxId
                let g = total_gravity(&entity.position); // m/s²

                // Convert m/s² → µm/s²
                let g_um = g * 1e6;

                // Acceleration is radial inward (negative)
                acc.ar = -g_um;

                // Angular accelerations unchanged
                acc.alat = 0.0;
                acc.alon = 0.0;

                // Emit event
                events.push(
                    ChronoEvent::accelerate(
                        entity.id,
                        entity.world_id,
                        world.sim_time,
                        acc.ar,
                        acc.alat,
                        acc.alon
                    )
                );
            }
        }

        events
    }
}

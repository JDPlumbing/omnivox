use crate::sim::systems::System;
use crate::sim::world::WorldState;
use crate::core::chronovox::{ChronoEvent, EventKind};

pub struct UVDegradationSystem;

impl System for UVDegradationSystem {
    fn name(&self) -> &'static str {
        "UVDegradationSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        for (entity_id, uv) in world.components.uv_degradation_components.iter_mut() {
            // Extremely simplified aging law:
            // More UV dose = less material strength
            let degrade_factor = (uv.total_uv_dose / 1.0e9).min(0.5); // cap at 50%

            if let Some(strength) = world.components.strength_components.get_mut(entity_id) {
                let original = strength.tensile_strength_mpa;
                strength.tensile_strength_mpa =
                original * (1.0 - degrade_factor) as f32;


                events.push(ChronoEvent {
                    entity_id: *entity_id,
                    world_id: world.meta.world_id,
                    t: world.sim_time,
                    kind: EventKind::Custom("UV_DegradationApplied".into()),
                    payload: None,
                });
            }
        }

        events
    }
}

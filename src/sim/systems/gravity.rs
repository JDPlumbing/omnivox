use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState, components::Acceleration},
    tdt::core::TimeDelta,
};
use tracing::info;

pub struct GravitySystem;

impl System for GravitySystem {
    fn name(&self) -> &'static str { "GravitySystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        const EARTH_GRAVITY: f64 = -9.81;

        for (entity_id_str, _obj) in world.objects.iter() {
            if let Ok(entity_id) = uuid::Uuid::parse_str(entity_id_str) {
                // Create or update acceleration with gravity
                let accel = world.acceleration_components
                    .entry(entity_id)
                    .and_modify(|a| a.ar += EARTH_GRAVITY)
                    .or_insert(Acceleration::new(EARTH_GRAVITY, 0.0, 0.0));


                events.push(ChronoEvent {
                    id: world.objects[entity_id_str].uvoxid.clone(),
                    t: TimeDelta::from_ticks(1, "nanoseconds"),
                    kind: EventKind::Accelerate {
                        ar: accel.ar,
                        alat: accel.alat,
                        alon: accel.alon,
                    },
                    payload: None,
                });
            }
        }

        events
    }
}

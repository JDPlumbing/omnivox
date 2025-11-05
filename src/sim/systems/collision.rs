use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState},
    tdt::core::TimeDelta,
};

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn name(&self) -> &'static str {
        "CollisionSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        const EARTH_RADIUS: f64 = 6_371_000_000.0;

        for (entity_id, obj) in world.objects.iter_mut() {
            // Simple ground collision test
            if (obj.uvoxid.r_um as f64) <= EARTH_RADIUS {
                // Stop all motion
                if let Some(v) = world.velocity_components.get_mut(&uuid::Uuid::parse_str(entity_id).unwrap()) {
                    v.dr = 0.0;
                    v.dlat = 0.0;
                    v.dlon = 0.0;
                }
                if let Some(a) = world.acceleration_components.get_mut(&uuid::Uuid::parse_str(entity_id).unwrap()) {
                    a.ar = 0.0;
                    a.alat = 0.0;
                    a.alon = 0.0;
                }

                // Snap to surface
                obj.uvoxid.r_um = EARTH_RADIUS as i64;

                // Emit collision event
                events.push(ChronoEvent {
                    id: obj.uvoxid.clone(),
                    t: TimeDelta::from_ticks(1, "nanoseconds"),
                    kind: EventKind::Custom("Collision".into()),
                    payload: None,
                });
            }
        }

        events
    }
}

use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState, environment::default_material_for_r_um, components::Velocity},
    tdt::core::TimeDelta,
};

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn name(&self) -> &'static str {
        "CollisionSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        const EARTH_RADIUS: i64 = 6_371_000_000_000;
        for (entity_id_str, obj) in world.objects.iter_mut() {
            if let Ok(entity_id) = uuid::Uuid::parse_str(entity_id_str) {
                if let Some(v) = world.velocity_components.get_mut(&entity_id) {
            // do your collision stuff
                    v.dr = 0.0;
                    v.dlat = 0.0;
                    v.dlon = 0.0;
                }

                // push a collision event
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

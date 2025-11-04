use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState, components::Velocity},
    tdt::core::TimeDelta,
};

pub struct MovementSystem;

impl System for MovementSystem {
    fn name(&self) -> &'static str {
        "MovementSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut triggered_events = Vec::new();

        // Iterate over all velocity components (only entities that can move)
        for (entity_id, velocity) in world.velocity_components.iter() {
            // Find matching object in the world
            if let Some(obj) = world.objects.get_mut(&entity_id.to_string()) {
                // Apply velocity to position (uvoxid encodes r_um, lat, lon)
                obj.uvoxid.r_um += velocity.dr as i64;
                obj.uvoxid.lat_code += velocity.dlat as i64;
                obj.uvoxid.lon_code += velocity.dlon as i64;

                // Emit movement event
                triggered_events.push(ChronoEvent {
                    id: obj.uvoxid.clone(),
                    t: TimeDelta::from_ticks(1, "nanoseconds"),
                    kind: EventKind::Move {
                        dr: velocity.dr as i64,
                        dlat: velocity.dlat as i64,
                        dlon: velocity.dlon as i64,
                    },
                    payload: None,
                });
            }
        }

        triggered_events
    }
}

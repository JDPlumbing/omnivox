use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState},
    tdt::{sim_time::SimTime, sim_duration::SimDuration},
};

pub struct AccelerationSystem;

impl System for AccelerationSystem {
    fn name(&self) -> &'static str {
        "AccelerationSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        // Pull time inputs from world
        let now = world.sim_time;
        let dt = world.sim_delta;

        for (entity_id, accel) in world.acceleration_components.iter() {
            if let Some(velocity) = world.velocity_components.get_mut(entity_id) {
                
                // Apply acceleration
                velocity.dr   += accel.ar;
                velocity.dlat += accel.alat;
                velocity.dlon += accel.alon;

                // Timestamp range
                let start = now;
                let end = now.add(dt);

                events.push(ChronoEvent {
                    id: world.objects
                        .get(&entity_id.to_string())
                        .map(|obj| obj.uvoxid.clone())
                        .unwrap_or_default(),
                    t: end,

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

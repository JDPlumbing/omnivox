use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState, components::Velocity},
    tdt::core::TimeDelta,
};
pub struct AccelerationSystem;

impl System for AccelerationSystem {
    fn name(&self) -> &'static str {
        "AccelerationSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        // Loop through all entities with acceleration
        for (entity_id, accel) in world.acceleration_components.iter() {
            tracing::info!("⚡ Found accel for entity {:?}: {:?}", entity_id, accel);

            if let Some(velocity) = world.velocity_components.get_mut(entity_id) {
                tracing::info!("🚀 Applying accel {:?} to velocity {:?}", accel, velocity);

                // Apply acceleration to velocity
                velocity.dr   += accel.ar;
                velocity.dlat += accel.alat;
                velocity.dlon += accel.alon;


                // Log the event
                events.push(ChronoEvent {
                    id: world
                        .objects
                        .get(&entity_id.to_string())
                        .map(|obj| obj.uvoxid.clone())
                        .unwrap_or_default(),
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
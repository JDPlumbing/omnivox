use crate::core::{
    chronovox::{ChronoEvent, EventKind},
    
    tdt::core::TimeDelta,
};
use crate::sim::{
        systems::System,
        world::WorldState,
        components::acceleration::Acceleration,
    };
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GravitySystem;

impl System for GravitySystem {
    fn name(&self) -> &'static str { "GravitySystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        const EARTH_GRAVITY: f64 = -9.81;

        let Some(clock) = &world.clock else {
            return events;
        };

        let dt = TimeDelta::from_sim_duration(clock.step);

        for (_key, obj) in world.objects.iter() {
            let entity_uuid = obj.entity_id;

            let accel = world.components.acceleration_components
                .entry(entity_uuid)
                .and_modify(|a| a.ar += EARTH_GRAVITY)
                .or_insert(Acceleration::new(EARTH_GRAVITY, 0.0, 0.0));

            events.push(ChronoEvent {
                id: obj.uvoxid.clone(),
                t: world.clock.as_ref().unwrap().current,


                kind: EventKind::Accelerate {
                    ar: accel.ar,
                    alat: accel.alat,
                    alon: accel.alon,
                },
                payload: None,
            });
        }

        events
    }
}

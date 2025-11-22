use crate::core::{
    chronovox::{ChronoEvent, EventKind},
};

use crate::sim::{
    systems::System,
    world::WorldState,
    components::{SunlightComponent, SolarExposureData},
};

use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SolarExposureSystem;

impl System for SolarExposureSystem {
    fn name(&self) -> &'static str { "SolarExposureSystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        let Some(clock) = &world.clock else {
            return events;
        };

        let now   = clock.current;
        let dt_s  = clock.step_seconds();

        //
        // Iterate over (entity_id, sunlight)
        //
        for (entity_id, sunlight) in world.components.sunlight_components.clone() {
            //
            // Get the actual entity
            //
            let Some(entity) = world.entities.get(&entity_id) else {
                continue; // stale component
            };

            //
            // Get or init exposure stats
            //
            let exposure = world
                .components
                .solar_exposure_components
                .entry(entity_id)
                .or_insert(SolarExposureData {
                    energy_j_m2: 0.0,
                    uv_j_m2: 0.0,
                });

            //
            // Energy accumulation
            //
            let radiant_energy = sunlight.irradiance_w_m2 * dt_s;
            exposure.energy_j_m2 += radiant_energy;

            //
            // UV accumulation
            //
            let uv_factor = sunlight.uv_index / 100.0;
            let uv_energy = radiant_energy * uv_factor;
            exposure.uv_j_m2 += uv_energy;

            //
            // Emit proper ChronoEvent
            //
            events.push(
                ChronoEvent::new(
                    entity.entity_id,
                    entity.world_id,
                    now,
                    EventKind::Custom("SolarExposureUpdate".into()),
                )
                .with_payload(json!({
                    "entity_id": entity.entity_id.to_string(),
                    "energy_j_m2": exposure.energy_j_m2,
                    "uv_j_m2":     exposure.uv_j_m2,
                    "timestamp":   clock.current_wall_time().to_rfc3339(),
                }))
            );
        }

        events
    }
}

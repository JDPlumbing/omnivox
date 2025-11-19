use crate::core::{
    chronovox::{ChronoEvent, EventKind},

    tdt::core::TimeDelta,
    tdt::sim_duration::SimDuration,      // <-- REQUIRED
};
use crate::sim::{systems::System, world::WorldState};
use crate::sim::components::{SunlightComponent, SolarExposureData},
use uuid::Uuid;
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
        let now = clock.current;
        let dt_s = clock.step_seconds();

        // Clone avoids borrow checker problems
        for (id, sunlight) in world.components.sunlight_components.clone() {

            let exposure = world.components.solar_exposure_components
                .entry(id)
                .or_insert(SolarExposureData {
                    energy_j_m2: 0.0,
                    uv_j_m2: 0.0,
                });

            // ------------------------------
            // Correct energy accumulation
            // ------------------------------
            let radiant_energy = sunlight.irradiance_w_m2 * dt_s;
            exposure.energy_j_m2 += radiant_energy;

            let uv_factor = sunlight.uv_index / 100.0;
            let uv_energy = radiant_energy * uv_factor;
            exposure.uv_j_m2 += uv_energy;

            // ------------------------------
            // Emit event
            // ------------------------------
            events.push(ChronoEvent {
                id: world.objects[&id.to_string()].uvoxid,

               t: now,

                kind: EventKind::Custom("SolarExposureUpdate".into()),
                payload: Some(json!({
                    "uuid": id.to_string(),
                    "energy_j_m2": exposure.energy_j_m2,
                    "uv_j_m2": exposure.uv_j_m2,
                    "timestamp": clock.current_wall_time().to_rfc3339(),
                })),
            });
        }

        events
    }
}

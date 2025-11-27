// src/sim/systems/solar_sky.rs
//! SolarSkySystem
//!
//! Reads Sun and observer positions → produces a SunlightComponent
//! and accumulates SolarExposure + UVDegradation.

use crate::core::chronovox::{ChronoEvent, EventKind};
use crate::core::tdt::SimTime;
use crate::sim::systems::System;
use crate::sim::world::WorldState;
use crate::sim::components::{
    SunEmitter,
    SunlightComponent,
    SolarExposure,
    UVDegradationData,
};
use crate::core::physox::astronomy::topocentric::sun_topocentric;

use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SolarSkySystem;

impl System for SolarSkySystem {
    fn name(&self) -> &'static str {
        "SolarSkySystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        let Some(clock) = &world.clock else {
            return events;
        };

        let t: SimTime = world.sim_time;

        // Find Sun entity
        let Some((&sun_id, sun_emitter)) =
            world.components.sun_emitter_components.iter().next()
        else {
            return events;
        };

        let Some(sun_obj) = world.entities.get(&sun_id) else {
            return events;
        };

        // -----------------------------------------
        // Compute solar topocentric geometry
        // -----------------------------------------
        for (entity_id, entity) in world.entities.iter() {
            if *entity_id == sun_id {
                continue;
            }

            let topo = sun_topocentric(entity.uvoxid, t);

            // Compute approximate irradiance
            let dist_m = sun_obj.uvoxid.r_um as f64 / 1e6;
            let irradiance_w_m2 =
                sun_emitter.luminosity_w / (4.0 * std::f64::consts::PI * dist_m.powi(2));

            let lit = topo.is_daylight;
            let irr_factor = topo.irradiance_factor;
            let uv_index = irr_factor * sun_emitter.uv_fraction * 10.0;

            if lit {
                world.components.sunlight_components.insert(
                    *entity_id,
                    SunlightComponent {
                        irradiance_factor: irr_factor,
                        elevation_deg: topo.elevation_deg,
                        azimuth_deg: topo.azimuth_deg,
                        is_daylight: true,
                        uv_index,
                        irradiance_w_m2,
                    },
                );

                // Update exposure component
                let dt_s = world.sim_delta.seconds_f64();

                let se = world
                    .components
                    .solar_exposure
                    .entry(*entity_id)
                    .or_insert_with(SolarExposure::new);

                se.irradiance_factor = irr_factor;
                se.cumulative_exposure += irr_factor * dt_s;
                se.cumulative_uv += uv_index * dt_s;

                // Event
                events.push(
                    ChronoEvent::custom(
                        *entity_id,
                        entity.world_id,
                        t,
                        format!(
                            "SunExposure elevation={:.2} az={:.2} irr={:.2}",
                            topo.elevation_deg,
                            topo.azimuth_deg,
                            irr_factor
                        ),
                    )
                    .with_payload(json!({
                        "entity_id": entity_id,
                        "is_daylight": lit,
                        "irradiance": irradiance_w_m2,
                        "elevation_deg": topo.elevation_deg,
                        "azimuth_deg": topo.azimuth_deg,
                        "uv_index": uv_index,
                    })),
                );
            } else {
                world.components.sunlight_components.remove(entity_id);
            }
        }

        events
    }
}

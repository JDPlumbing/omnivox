use crate::core::{
    chronovox::{ChronoEvent, EventKind},
};
use crate::sim::{
    systems::System,
    world::WorldState,
    components::{SunEmitter, SunlightComponent},
};

use serde_json::json;
use serde::{Serialize, Deserialize};
use std::f64::consts::PI;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SolarRaycastSystem;

impl System for SolarRaycastSystem {
    fn name(&self) -> &'static str {
        "SolarRaycastSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        // Need a clock
        let Some(clock) = &world.clock else {
            return events;
        };
        let now = clock.current;

        // -------------------------------------------------------------
        // STEP 1 — Find the Sun entity & emitter component
        // -------------------------------------------------------------
        let Some((&sun_id, sun_emitter)) =
            world.components.sun_emitter_components.iter().next()
        else {
            return events;
        };

        let Some(sun_obj) = world.entities.get(&sun_id) else {
            return events;
        };

        // Convert sun position to radians
        let sun_lat_rad = (sun_obj.uvoxid.lat_code as f64 / 1e11).to_radians();
        let sun_lon_rad = (sun_obj.uvoxid.lon_code as f64 / 1e11).to_radians();

        // Distance Sun→Earth center (meters)
        let r_m = sun_obj.uvoxid.r_um as f64 / 1e6;

        // Radiant flux at that distance
        let irradiance_w_m2 =
            sun_emitter.luminosity_w / (4.0 * PI * r_m.powi(2));

        // -------------------------------------------------------------
        // STEP 2 — For every entity, compute solar exposure
        // -------------------------------------------------------------
        for (id, obj) in world.entities.iter() {
            if *id == sun_id {
                continue;
            }

            // Object surface position
            let lat_rad = (obj.uvoxid.lat_code as f64 / 1e11).to_radians();
            let lon_rad = (obj.uvoxid.lon_code as f64 / 1e11).to_radians();

            // Solar zenith angle
            let cos_z =
                sun_lat_rad.sin() * lat_rad.sin() +
                sun_lat_rad.cos() * lat_rad.cos() *
                (sun_lon_rad - lon_rad).cos();

            let zenith_rad = cos_z.acos();
            let zenith_deg = zenith_rad.to_degrees();

            let is_daylight = zenith_deg < 90.0;

            // Elevation = 90 - zenith
            let elevation_deg = 90.0 - zenith_deg;
            let elevation_rad = elevation_deg.to_radians();

            // Azimuth (simplified horizontal solar azimuth)
            // Based on spherical trig formula
            let y = (sun_lon_rad - lon_rad).sin();
            let x = (sun_lat_rad * lat_rad.tan().cos())
                  - (lat_rad.sin() * (sun_lon_rad - lon_rad).cos());

            let mut azimuth_deg = y.atan2(x).to_degrees();
            if azimuth_deg < 0.0 {
                azimuth_deg += 360.0;
            }

            // Cosine law for irradiance
            let irradiance_factor = elevation_rad.sin().max(0.0);

            // ---------------------------------------------------------
            // STEP 3 — Apply sunlight component
            // ---------------------------------------------------------
            if is_daylight {
                world.components.sunlight_components.insert(
                    *id,
                    SunlightComponent {
                        irradiance_factor,
                        elevation_deg,
                        azimuth_deg,
                        is_daylight,

                        // compatibility fields
                        irradiance_w_m2,
                        uv_index: sun_emitter.uv_fraction * 100.0,
                    }
                );
            } else {
                world.components.sunlight_components.remove(id);
            }

            // ---------------------------------------------------------
            // STEP 4 — Emit event for debugging/logging
            // ---------------------------------------------------------
            events.push(
                ChronoEvent::new(
                    obj.entity_id,
                    obj.world_id,
                    now,
                    EventKind::Custom("SolarRaycastUpdate".into())
                )
                .with_payload(json!({
                    "entity_id": id,
                    "zenith_angle_deg": zenith_deg,
                    "elevation_deg": elevation_deg,
                    "azimuth_deg": azimuth_deg,
                    "irradiance_factor": irradiance_factor,
                    "lit": is_daylight,
                    "timestamp": clock.current_wall_time().to_rfc3339(),
                }))
            );
        }

        events
    }
}

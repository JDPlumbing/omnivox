use crate::core::{
    chronovox::{ChronoEvent, EventKind},
};
use crate::sim::{
    systems::System,
    world::WorldState,
    components::{SunEmitter, SunlightComponent},
};

use serde_json::json;
use std::f64::consts::PI;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SolarRaycastSystem;

impl System for SolarRaycastSystem {
    fn name(&self) -> &'static str {
        "SolarRaycastSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        let Some(clock) = &world.clock else {
            return events;
        };
        let now = clock.current;

        // -------------------------------------------------------------
        // STEP 1 — Find the entity that acts as the Sun
        // -------------------------------------------------------------
        let Some((&sun_id, sun_emitter)) =
            world.components.sun_emitter_components.iter().next()
        else {
            // No sun in the world
            return events;
        };

        let Some(sun_obj) = world.entities.get(&sun_id) else {
            return events;
        };

        // -------------------------------------------------------------
        // STEP 2 — Compute sun position in spherical coords
        // -------------------------------------------------------------
        let sun_lat = (sun_obj.uvoxid.lat_code as f64 / 1e11).to_radians();
        let sun_lon = (sun_obj.uvoxid.lon_code as f64 / 1e11).to_radians();

        // Distance from Earth center (meters)
        let r_m = sun_obj.uvoxid.r_um as f64 / 1e6;

        // Irradiance at distance r
        let irradiance = sun_emitter.luminosity_w / (4.0 * PI * r_m.powi(2));

        // -------------------------------------------------------------
        // STEP 3 — For each entity, compute light exposure
        // -------------------------------------------------------------
        for (id, obj) in world.entities.iter() {
            // Skip the sun itself
            if *id == sun_id {
                continue;
            }

            let lat = (obj.uvoxid.lat_code as f64 / 1e11).to_radians();
            let lon = (obj.uvoxid.lon_code as f64 / 1e11).to_radians();

            // Solar zenith angle
            let cos_z =
                sun_lat.sin() * lat.sin() +
                sun_lat.cos() * lat.cos() * (sun_lon - lon).cos();

            let zenith_deg = cos_z.acos().to_degrees();
            let is_lit = zenith_deg < 90.0;

            // ---------------------------------------------------------
            // STEP 4 — Set or clear sunlight component
            // ---------------------------------------------------------
            if is_lit {
                world.components.sunlight_components.insert(
                    *id,
                    SunlightComponent {
                        irradiance_w_m2: irradiance,
                        uv_index: sun_emitter.uv_fraction * 100.0,
                    }
                );
            } else {
                world.components.sunlight_components.remove(id);
            }

            // ---------------------------------------------------------
            // STEP 5 — Emit solar update event
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
                    "lit": is_lit,
                    "zenith_angle_deg": zenith_deg,
                    "timestamp": clock.current_wall_time().to_rfc3339(),
                }))
            );
        }

        return events;
    }
}

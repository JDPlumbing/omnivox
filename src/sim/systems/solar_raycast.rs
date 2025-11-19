use crate::core::{
    chronovox::{ChronoEvent, EventKind},
    tdt::sim_duration::SimDuration;
    tdt::core::TimeDelta,
};
use crate::sim::{
    {systems::System, world::WorldState},
    components::{SunEmitter, SunlightComponent},};
use uuid::Uuid;
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
        let dt_s = world.clock.as_ref().unwrap().step_seconds();
        let now = clock.current;

        // --- FIND SUN OBJECT ---
        let (sun_id_str, sun_obj) = match world.objects.iter().find(|(_, o)| o.name == "Sun") {
            Some(pair) => pair,
            None => return events,
        };

        let sun_uuid = match Uuid::parse_str(sun_id_str) {
            Ok(id) => id,
            Err(_) => return events,
        };

        // ---------------------------------------------------------
        // 🔥 FIXED: read SunEmitter (source of luminosity/UV/etc)
        // ---------------------------------------------------------
        let Some(emitter) = world.components.sun_emitter_components.get(&sun_uuid) else {
            return events;
        };

        // Convert position
        let sun_lat = (sun_obj.uvoxid.lat_code as f64 / 1e11).to_radians();
        let sun_lon = (sun_obj.uvoxid.lon_code as f64 / 1e11).to_radians();

        let r_m = sun_obj.uvoxid.r_um as f64 / 1e6;

        // ---------------------------------------------------------
        // 🔥 FIXED: irradiance comes from SunEmitter.luminosity_w
        // ---------------------------------------------------------
        let irradiance = emitter.luminosity_w / (4.0 * PI * r_m.powi(2));

        // --- RAYCAST TO ALL OBJECTS ---
        for (id_str, obj) in &world.objects {
            if obj.name == "Sun" {
                continue;
            }

            let uuid = match Uuid::parse_str(id_str) {
                Ok(id) => id,
                Err(_) => continue,
            };

            let lat = (obj.uvoxid.lat_code as f64 / 1e11).to_radians();
            let lon = (obj.uvoxid.lon_code as f64 / 1e11).to_radians();

            // Solar zenith geometry
            let cos_z =
                sun_lat.sin() * lat.sin() +
                sun_lat.cos() * lat.cos() * (sun_lon - lon).cos();

            let zenith = cos_z.acos().to_degrees();
            let lit = zenith < 90.0;

            if lit {
                // ---------------------------------------------------------
                // 🔥 FIXED: write per-object sunlight component
                // ---------------------------------------------------------
                world.components.sunlight_components.insert(uuid, SunlightComponent {
                    irradiance_w_m2: irradiance,
                    uv_index: emitter.uv_fraction * 100.0,
                });

            } else {
                world.components.sunlight_components.remove(&uuid);
            }

            events.push(ChronoEvent {
                id: obj.uvoxid,

              t: now,

                kind: EventKind::Custom("SolarRaycastUpdate".into()),
                payload: Some(json!({
                    "uuid": id_str,
                    "lit": lit,
                    "zenith_angle_deg": zenith,
                    "timestamp": clock.current_wall_time().to_rfc3339()

                })),
            });

        }

        events
    }
}

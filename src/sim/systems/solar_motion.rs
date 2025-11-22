use crate::core::{
    chronovox::{ChronoEvent, EventKind},
};
use crate::sim::{
    systems::System,
    world::WorldState,
    components::OrbitalMotion,
};

use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SolarMotionSystem;

impl System for SolarMotionSystem {
    fn name(&self) -> &'static str { "SolarMotionSystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let Some(clock) = &world.clock else { return vec![] };
        let dt_s = clock.step_seconds();
        let now = clock.current;
        let mut events = vec![];

        //---------------------------------------------------------------
        // STEP 1 — Locate the Sun entity
        // Rule: entity.metadata["name"] == "Sun"
        //---------------------------------------------------------------
        let maybe_sun = world.entities
            .iter_mut()
            .find(|(_, ent)| {
                ent.metadata
                    .get("name")
                    .and_then(|v| v.as_str())
                    == Some("Sun")
            });

        let (sun_id, sun) = match maybe_sun {
            Some(pair) => pair,
            None => return events, // No sun in this world
        };

        //---------------------------------------------------------------
        // STEP 2 — Get its orbital motion component
        //---------------------------------------------------------------
        let Some(orbit) = world.components.orbital_components.get_mut(sun_id) else {
            return events;
        };

        //---------------------------------------------------------------
        // STEP 3 — Convert integer fields to f64 for math
        //---------------------------------------------------------------
        let lon_rate = orbit.lon_rate_per_s as f64;
        let lat_rate = orbit.lat_rate_per_s as f64;
        let r_rate   = orbit.r_rate_per_s as f64;

        //---------------------------------------------------------------
        // Step 4 — Longitude motion (360° wrap)
        //---------------------------------------------------------------
        const DEG_SCALE: f64 = 1e11;       // uvox degrees scaling
        const FULL_ROT: f64 = 360.0 * DEG_SCALE;

        let new_lon = (sun.uvoxid.lon_code as f64 + lon_rate * dt_s)
            .rem_euclid(FULL_ROT);
        sun.uvoxid.lon_code = new_lon as i64;

        //---------------------------------------------------------------
        // Step 5 — Latitude oscillation (seasonal tilt)
        //---------------------------------------------------------------
        let mut new_lat =
            sun.uvoxid.lat_code as f64 + lat_rate * dt_s * orbit.tilt_dir as f64;

        if new_lat.abs() > orbit.lat_amp as f64 {
            orbit.tilt_dir *= -1;
            // Reapply using reversed direction so we stay within bounds
            new_lat =
                sun.uvoxid.lat_code as f64 + lat_rate * dt_s * orbit.tilt_dir as f64;
        }

        sun.uvoxid.lat_code = new_lat as i64;

        //---------------------------------------------------------------
        // Step 6 — Radius oscillation (eccentric orbit)
        //---------------------------------------------------------------
        let mut new_r =
            sun.uvoxid.r_um as f64 + r_rate * dt_s * orbit.r_dir as f64;

        let max_r = (orbit.mean_r_um + orbit.delta_r_um) as f64;
        let min_r = (orbit.mean_r_um - orbit.delta_r_um) as f64;

        if new_r >= max_r || new_r <= min_r {
            orbit.r_dir *= -1;
            // recompute with flipped direction
            new_r =
                sun.uvoxid.r_um as f64 + r_rate * dt_s * orbit.r_dir as f64;
        }

        sun.uvoxid.r_um = new_r as i64;

        //---------------------------------------------------------------
        // Step 7 — Emit ChronoEvent in modern format
        //---------------------------------------------------------------
        events.push(
            ChronoEvent::new(
                sun.entity_id,
                sun.world_id,
                now,
                EventKind::Custom("SolarPositionUpdate".into()),
            )
            .with_payload(json!({
                "entity_id": sun.entity_id,
                "lat_code": sun.uvoxid.lat_code,
                "lon_code": sun.uvoxid.lon_code,
                "r_um": sun.uvoxid.r_um
            }))
        );

        events
    }
}

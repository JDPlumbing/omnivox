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

use crate::core::uvoxid::{RUm, LatCode, LonCode};

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
            None => return events,
        };

        //---------------------------------------------------------------
        // STEP 2 — Get orbital component
        //---------------------------------------------------------------
        let Some(orbit) = world.components.orbital_components.get_mut(&sun_id) else {
            return events;
        };

        //---------------------------------------------------------------
        // STEP 3 — Pull motion rates as f64
        //---------------------------------------------------------------
        let lon_rate = orbit.lon_rate_per_s as f64;
        let lat_rate = orbit.lat_rate_per_s as f64;
        let r_rate   = orbit.r_rate_per_s as f64;

        //---------------------------------------------------------------
        // STEP 4 — LONGITUDE UPDATE
        //---------------------------------------------------------------
        const DEG_SCALE: f64 = 1e11;
        const FULL_ROT: f64 = 360.0 * DEG_SCALE;

        let current_lon = sun.position.lon_code.0 as f64;

        let new_lon_val =
            (current_lon + lon_rate * dt_s).rem_euclid(FULL_ROT);

        sun.position.lon_code = LonCode(new_lon_val as i64);

        //---------------------------------------------------------------
        // STEP 5 — LATITUDE UPDATE (seasonal oscillation)
        //---------------------------------------------------------------
        let current_lat = sun.position.lat_code.0 as f64;

        let mut new_lat_val =
            current_lat + lat_rate * dt_s * orbit.tilt_dir as f64;

        if new_lat_val.abs() > orbit.lat_amp as f64 {
            orbit.tilt_dir *= -1;
            new_lat_val =
                current_lat + lat_rate * dt_s * orbit.tilt_dir as f64;
        }

        sun.position.lat_code = LatCode(new_lat_val as i64);

        //---------------------------------------------------------------
        // STEP 6 — RADIUS UPDATE (eccentric orbit)
        //---------------------------------------------------------------
        let current_r = sun.position.r_um.0 as f64;

        let mut new_r_val =
            current_r + r_rate * dt_s * orbit.r_dir as f64;

        let max_r = (orbit.mean_r_um + orbit.delta_r_um) as f64;
        let min_r = (orbit.mean_r_um - orbit.delta_r_um) as f64;

        if new_r_val >= max_r || new_r_val <= min_r {
            orbit.r_dir *= -1;
            new_r_val =
                current_r + r_rate * dt_s * orbit.r_dir as f64;
        }

        sun.position.r_um = RUm(new_r_val as i64);

        //---------------------------------------------------------------
        // STEP 7 — Emit modern ChronoEvent
        //---------------------------------------------------------------
        events.push(
            ChronoEvent::new(
                sun.id,
                sun.world_id,
                now,
                EventKind::Custom("SolarPositionUpdate".into()),
            )
            .with_payload(json!({
                "entity_id": sun.id,
                "lat_code": sun.position.lat_code.0,
                "lon_code": sun.position.lon_code.0,
                "r_um": sun.position.r_um.0,
            }))
        );

        events
    }
}

use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState},
    sim::components::OrbitalMotion,
    tdt::core::TimeDelta,
};
use serde_json::json;
use uuid::Uuid;
use crate::tdt::sim_duration::SimDuration;

pub struct SolarMotionSystem;

impl System for SolarMotionSystem {
    fn name(&self) -> &'static str { "SolarMotionSystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let Some(clock) = &world.clock else { return vec![] };
        let dt_s = clock.step_seconds();
        let now = clock.current;
        let mut events = vec![];

        // --- locate the Sun ---
        let (id_str, obj) = match world.objects.iter_mut().find(|(_, o)| o.name == "Sun") {
            Some(p) => p,
            None => return events,
        };

        let uuid = match Uuid::parse_str(id_str) {
            Ok(v) => v,
            Err(_) => return events,
        };

        let Some(orbit) = world.orbital_components.get_mut(&uuid) else {
            return events;
        };

        // -------------------------------
        // Convert integer fields to f64
        // -------------------------------
        let lon_rate = orbit.lon_rate_per_s as f64;
        let lat_rate = orbit.lat_rate_per_s as f64;
        let r_rate   = orbit.r_rate_per_s as f64;

        // -------------------------------
        // LONGITUDE UPDATE
        // uvox uses 1e11 units per degree
        // -------------------------------
        const DEG_SCALE: f64 = 1e11;
        const FULL_ROT: f64 = 360.0 * DEG_SCALE;

        let new_lon = (obj.uvoxid.lon_code as f64 + lon_rate * dt_s)
            .rem_euclid(FULL_ROT);

        obj.uvoxid.lon_code = new_lon as i64;

        // -------------------------------
        // LATITUDE OSCILLATION (tilt)
        // -------------------------------
        let new_lat =
            obj.uvoxid.lat_code as f64 + lat_rate * dt_s * orbit.tilt_dir as f64;

        // flip direction at amplitude bounds
        if new_lat.abs() > orbit.lat_amp as f64 {
            orbit.tilt_dir *= -1;
        }

        obj.uvoxid.lat_code = new_lat as i64;

        // -------------------------------
        // RADIUS UPDATE (eccentric orbit)
        // r_um is stored in micrometers
        // -------------------------------
        let new_r =
            obj.uvoxid.r_um as f64 + r_rate * dt_s * orbit.r_dir as f64;

        if new_r >= (orbit.mean_r_um + orbit.delta_r_um) as f64 ||
           new_r <= (orbit.mean_r_um - orbit.delta_r_um) as f64
        {
            orbit.r_dir *= -1;
        }

        obj.uvoxid.r_um = new_r as i64;

        // -------------------------------
        // Emit event
        // -------------------------------
        events.push(ChronoEvent {
            id: obj.uvoxid,
            t: now,
            kind: EventKind::Custom("SolarPositionUpdate".into()),
            payload: Some(json!({
                "lat_code": obj.uvoxid.lat_code,
                "lon_code": obj.uvoxid.lon_code,
                "r_um": obj.uvoxid.r_um,
            })),
        });

        events
    }
}

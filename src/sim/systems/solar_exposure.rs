use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState, clock::SimClock},
    tdt::core::TimeDelta,
};
use uuid::Uuid;
use serde_json::json;
use chrono::Datelike;

#[derive(Debug, Clone)]
pub struct SolarExposureData {
    pub cumulative_irradiance_j_m2: f64,
    pub cumulative_uv_j_m2: f64,
    pub cycle_count: u32,
}

pub struct SolarExposureSystem;

impl System for SolarExposureSystem {
    fn name(&self) -> &'static str {
        "SolarExposureSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        let Some(clock) = &world.clock else { return events };

        let dt_s = clock.step.num_seconds();

        for (entity_id_str, _) in &world.objects {
            let uuid = match Uuid::parse_str(entity_id_str) {
                Ok(id) => id,
                Err(_) => continue,
            };

            let Some(sun) = world.sunlight_components.get(&uuid) else { continue; };

            // Retrieve or initialize accumulated exposure
            let entry = world.solar_exposure_components.entry(uuid).or_insert_with(|| SolarExposureData {
                cumulative_irradiance_j_m2: 0.0,
                cumulative_uv_j_m2: 0.0,
                cycle_count: 0,
            });

            // Integrate sunlight energy over this step
            // Compute latitude-dependent daylight fraction using solar declination
            let lat_deg = world.objects.get(entity_id_str)
                .map(|o| o.uvoxid.lat_code as f64 / 1e6)
                .unwrap_or(0.0);

            let day_of_year = clock.current.ordinal() as f64;
            let decl = (23.44 * (2.0 * std::f64::consts::PI * (day_of_year - 81.0) / 365.0).sin()).to_radians();
            let lat = lat_deg.to_radians();

            // Hour angle for sunrise/sunset
            let cos_omega = (-lat.tan() * decl.tan()).clamp(-1.0, 1.0);
            let daylight_fraction = (1.0 / std::f64::consts::PI) * (cos_omega.acos() * 2.0);
            let daylight_fraction = daylight_fraction.clamp(0.0, 1.0);


            entry.cumulative_irradiance_j_m2 += sun.irradiance_w_m2 * dt_s as f64 * daylight_fraction;
            entry.cumulative_uv_j_m2 += (sun.uv_index as f64 * 5.0) * dt_s as f64 * daylight_fraction;


            // Count day/night cycles based on elapsed simulated days
            entry.cycle_count += (clock.step.num_days().max(1)) as u32;

            events.push(ChronoEvent {
                id: crate::uvoxid::UvoxId::default(),
                t: TimeDelta::from_ticks(dt_s, "seconds"),
                kind: EventKind::Custom("SolarExposureUpdate".into()),
                payload: Some(json!({
                    "date": clock.current.to_rfc3339(),
                    "irradiance_total_j_m2": entry.cumulative_irradiance_j_m2,
                    "irradiance_total_kj_m2": entry.cumulative_irradiance_j_m2 / 1000.0,
                    "uv_total_j_m2": entry.cumulative_uv_j_m2,
                    "cycle_count": entry.cycle_count,
                })),
            });
        }

        events
    }
}

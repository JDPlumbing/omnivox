use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState, clock::SimClock},
    tdt::core::TimeDelta,
};
use chrono::{Datelike, Timelike};
use serde_json::json;
use std::f64::consts::PI;
use uuid::Uuid;

pub struct SunSystem;

#[derive(Debug, Clone)]
pub struct SunlightProps {
    pub irradiance_w_m2: f64,
    pub uv_index: f64,
    pub temperature_c: f64,
}

impl SunSystem {
    fn declination(day_of_year: f64) -> f64 {
        // tilt of Earth's axis (approx)
        (23.44 * (2.0 * PI * (day_of_year - 81.0) / 365.0).sin()).to_radians()
    }

    fn hour_angle(hour: f64, lon_deg: f64) -> f64 {
        // degrees → radians, 15° per hour, + longitude offset
        (15.0 * (hour - 12.0) + lon_deg).to_radians()
    }

    fn solar_altitude(lat_deg: f64, decl: f64, ha: f64) -> f64 {
        let lat = lat_deg.to_radians();
        (lat.sin() * decl.sin() + lat.cos() * decl.cos() * ha.cos()).asin()
    }

    fn temperature_with_lag(
        local_hour: f64,
        daylight_fraction: f64,
        lat_deg: f64
    ) -> f64 {
        // Parameters
        let phase_lag_hours = 3.0;          // peak temperature ~3 PM
        let base_temp = -20.0;              // coldest baseline
        let amplitude = 25.0 * lat_deg.cos(); // reduced amplitude toward poles

        // Shift hour so peak is 15:00 instead of 12:00
        let shifted_hour = (local_hour - phase_lag_hours + 24.0) % 24.0;

        // Sine curve between 6 AM and 6 PM
        let thermal_wave = ((std::f64::consts::PI * (shifted_hour - 6.0) / 12.0).sin()).max(0.0);

        base_temp + amplitude * thermal_wave * daylight_fraction
    }

    fn temperature_with_lag_and_season(
        local_hour: f64,
        daylight_fraction: f64,
        lat_deg: f64,
        day_of_year: f64
    ) -> f64 {
        // Daily phase lag — hottest around 3 PM
        let phase_lag_hours = 3.0;
        let shifted_hour = (local_hour - phase_lag_hours + 24.0) % 24.0;

        // Seasonal offset — warmest around day 200 (mid-July in N hemisphere)
        let seasonal_phase = (2.0 * std::f64::consts::PI * (day_of_year - 200.0) / 365.0).sin();
        let seasonal_bias = 10.0 * seasonal_phase * lat_deg.cos(); // ±10 °C seasonal swing

        // Base + amplitude
        let base_temp = -20.0 + seasonal_bias;
        let amplitude = 25.0 * lat_deg.cos();

        // Daily sine-wave for heating
        let thermal_wave = ((std::f64::consts::PI * (shifted_hour - 6.0) / 12.0).sin()).max(0.0);

        base_temp + amplitude * thermal_wave * daylight_fraction
    }
}

impl System for SunSystem {
    fn name(&self) -> &'static str {
        "SunSystem"
    }
    
    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        let Some(clock) = &world.clock else { return events };

        let date = clock.current;
        let day_of_year = date.ordinal() as f64;
        let hour = date.hour() as f64 + date.minute() as f64 / 60.0;

        const SOLAR_CONSTANT: f64 = 1361.0; // W/m² (top of atmosphere)

        for (entity_id, obj) in world.objects.iter() {
            let lat_deg = obj.uvoxid.lat_code as f64 / 1e6; // assuming µdeg precision
            let lon_deg = obj.uvoxid.lon_code as f64 / 1e6;

            let decl = Self::declination(day_of_year);
            let lat = lat_deg.to_radians();

            // fraction of day when Sun is above the horizon
            let cos_omega = (-lat.tan() * decl.tan()).clamp(-1.0, 1.0);
            let daylight_fraction = (1.0 / std::f64::consts::PI) * (cos_omega.acos() * 2.0);

            // average daily irradiance (W/m²)
            let irradiance_w_m2 = SOLAR_CONSTANT * daylight_fraction.max(0.0);

            // approximate daily UV exposure (scaled)
            let uv_index = 10.0 * daylight_fraction.powf(1.3);
            let temperature_c =
                Self::temperature_with_lag_and_season(hour, daylight_fraction, lat_deg, day_of_year);




            if let Ok(uuid) = Uuid::parse_str(entity_id) {
                world.sunlight_components.insert(
                    uuid,
                    SunlightProps {
                        irradiance_w_m2,
                        uv_index,
                        temperature_c,
                    },
                );
            }

            events.push(ChronoEvent {
                id: obj.uvoxid.clone(),
                t: TimeDelta::from_ticks(clock.step.num_seconds(), "seconds"),
                kind: EventKind::Custom("SunlightUpdate".into()),
                payload: Some(json!({
                    "date": date.to_rfc3339(),
                    "irradiance_w_m2": irradiance_w_m2,
                    "uv_index": uv_index,
                    "temperature_c": temperature_c,
                })),
            });
        }

        events
    }
}

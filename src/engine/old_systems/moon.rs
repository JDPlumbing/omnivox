use chrono::{Datelike, Timelike};
use serde::{Serialize, Deserialize};
use std::f64::consts::PI;

/// Rough constants for the Moon
const MOON_ALBEDO: f64 = 0.12;          // reflectivity of lunar surface
const EARTH_MOON_DIST_KM: f64 = 384_400.0;
const SOLAR_CONSTANT: f64 = 1361.0;     // W/m² incoming solar flux
const LUNAR_CYCLE_DAYS: f64 = 29.53059; // synodic month
const MOON_MASS_KG: f64 = 7.347e22;
const EARTH_RADIUS_M: f64 = 6.371e6;
const G: f64 = 6.67430e-11;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Moon {
    pub phase: f64,              // 0.0–1.0 new→full
    pub distance_km: f64,        // from Earth center
    pub irradiance_w_m2: f64,    // reflected sunlight at surface
    pub tidal_force_n_per_kg: f64,
}

impl Moon {
    pub fn new() -> Self {
        Self {
            phase: 0.0,
            distance_km: EARTH_MOON_DIST_KM,
            irradiance_w_m2: 0.0,
            tidal_force_n_per_kg: 0.0,
        }
    }

    pub fn update(&mut self, day_of_year: f64) {
        // approximate phase cycle (0..1)
        self.phase = (day_of_year % LUNAR_CYCLE_DAYS) / LUNAR_CYCLE_DAYS;

        // reflected light as simple cosine of phase
        let phase_angle = 2.0 * PI * self.phase;
        self.irradiance_w_m2 =
            SOLAR_CONSTANT * MOON_ALBEDO * 0.1 * (1.0 - phase_angle.cos()) / 2.0;

        // tidal force magnitude per unit mass
        let r_m = self.distance_km * 1_000.0;
        self.tidal_force_n_per_kg =
            2.0 * G * MOON_MASS_KG * EARTH_RADIUS_M / r_m.powi(3);
    }
}

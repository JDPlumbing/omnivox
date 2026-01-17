// core/env/derived_env.rs

use crate::core::env::env_snapshot::EnvSnapshot;
use crate::core::env::medium::Medium;

#[derive(Debug, Clone)]
pub struct DerivedEnv {
    pub medium: Medium,
    pub density: f64,
    pub gravity_radial: f64,
    pub pressure: f64,
    pub temperature: f64,
}

impl DerivedEnv {
    pub fn from_snapshot(s: EnvSnapshot) -> Self {
        let medium = if s.density > 0.0 {
            Medium::Gas
        } else {
            s.medium
        };

        const AMBIENT_PRESSURE_SCALE: f64 = 1.0;
        let pressure = s.density * s.gravity_radial * AMBIENT_PRESSURE_SCALE;

        Self {
            medium,
            density: s.density,
            gravity_radial: s.gravity_radial,
            pressure,
            temperature: s.temperature,
        }
    }
}

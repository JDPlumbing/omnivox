//! Definitions of global environment bodies (Earth, Sun, Moon)

use crate::core::uvoxid::{UvoxId, RUm};

/// Universal gravitational constant (m^3 / kg s^2)
pub const G: f64 = 6.67430e-11;

/// Environment body representing a celestial mass
#[derive(Debug, Clone, Copy)]
pub struct EnvironmentBody {
    pub name: &'static str,
    pub mass_kg: f64,
    pub radius_m: f64,
}

/// Earth model (WGS84-ish)
pub const EARTH: EnvironmentBody = EnvironmentBody {
    name: "Earth",
    mass_kg: 5.972e24,
    radius_m: 6_371_000.0,
};

/// Sun model
pub const SUN: EnvironmentBody = EnvironmentBody {
    name: "Sun",
    mass_kg: 1.98847e30,
    radius_m: 695_700_000.0,
};

/// Moon model
pub const MOON: EnvironmentBody = EnvironmentBody {
    name: "Moon",
    mass_kg: 7.34767309e22,
    radius_m: 1_737_400.0,
};

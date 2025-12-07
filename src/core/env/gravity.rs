//! Gravity models for Earth, Sun, Moon.
//!
//! Produces acceleration in m/s².

use crate::core::env::bodies::*;
use crate::core::uvoxid::UvoxId;

/// Compute gravitational acceleration GM / r²
#[inline]
fn grav_accel(mass_kg: f64, r_m: f64) -> f64 {
    G * mass_kg / (r_m * r_m)
}

/// Earth gravity at the entity’s radius
pub fn earth_gravity(id: &UvoxId) -> f64 {
    grav_accel(EARTH.mass_kg, id.r_um.meters())
}

/// Sun gravity at Earth distance (tidal contribution)
pub fn sun_gravity(_: &UvoxId) -> f64 {
    // TODO: replace with Sun’s dynamic position when solar body is implemented.
    const AU_M: f64 = 149_597_870_700.0;
    grav_accel(SUN.mass_kg, AU_M)
}

/// Moon tidal gravity
pub fn moon_gravity(_: &UvoxId) -> f64 {
    const MOON_DISTANCE_M: f64 = 384_400_000.0;
    grav_accel(MOON.mass_kg, MOON_DISTANCE_M)
}

/// Total effective gravitational acceleration
pub fn total_gravity(id: &UvoxId) -> f64 {
    earth_gravity(id) + sun_gravity(id) + moon_gravity(id)
}

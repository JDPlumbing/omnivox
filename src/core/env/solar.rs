//! Solar radiation and flux model
//!
//! Follows inverse-square law: flux = L / (4 π r²)

use std::f64::consts::PI;
use crate::core::env::bodies::{SUN, EARTH};
use crate::core::uvoxid::UvoxId;

/// Solar luminosity (W)
pub const SOLAR_LUMINOSITY_W: f64 = 3.828e26;

/// Solar constant at Earth distance (≈1361 W/m²)
pub fn solar_flux_at_earth() -> f64 {
    let r = 149_597_870_700.0; // 1 AU
    SOLAR_LUMINOSITY_W / (4.0 * PI * r * r)
}

/// Actual solar flux at entity position
pub fn solar_flux(id: &UvoxId) -> f64 {
    let r_m = id.r_um.meters();
    SOLAR_LUMINOSITY_W / (4.0 * PI * r_m * r_m)
}

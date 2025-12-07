//! Simple atmospheric model
//!
//! Returns:
//!   - density kg/m³ (approx)
//!   - classification of altitude band

use crate::core::env::bodies::EARTH;
use crate::core::uvoxid::UvoxId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtmosphereLayer {
    Surface,
    Troposphere,
    Stratosphere,
    Mesosphere,
    Thermosphere,
    Exosphere,
    Vacuum,
}

pub fn classify_layer(id: &UvoxId) -> AtmosphereLayer {
    let r_m = id.r_um.meters();
    let alt = r_m - EARTH.radius_m;

    match alt {
        x if x < 0.0 => AtmosphereLayer::Surface,
        x if x < 12_000.0 => AtmosphereLayer::Troposphere,
        x if x < 50_000.0 => AtmosphereLayer::Stratosphere,
        x if x < 80_000.0 => AtmosphereLayer::Mesosphere,
        x if x < 600_000.0 => AtmosphereLayer::Thermosphere,
        x if x < 1_000_000.0 => AtmosphereLayer::Exosphere,
        _ => AtmosphereLayer::Vacuum,
    }
}

/// Crude density curve, still useful for drag simulation
pub fn air_density(id: &UvoxId) -> f64 {
    let r_m = id.r_um.meters();
    let alt = r_m - EARTH.radius_m;

    if alt <= 0.0 {
        return 1.225; // sea level
    }

    // Exponential falloff model ρ = ρ0 * exp(-h / H)
    const SCALE_HEIGHT: f64 = 8500.0;
    1.225 * (-alt / SCALE_HEIGHT).exp()
}

//! Astronomy constants for solar/lunar ephemeris.
//!
//! Accuracy target: ~1° angular precision (good for lighting, seasons,
//! solar exposure, raycasting, and visual simulation).

/// PI constant (f64)
pub const PI: f64 = std::f64::consts::PI;

/// Degrees → radians
#[inline]
pub fn deg2rad(d: f64) -> f64 {
    d * PI / 180.0
}

/// Radians → degrees
#[inline]
pub fn rad2deg(r: f64) -> f64 {
    r * 180.0 / PI
}

/// Earth equatorial radius (m)
pub const EARTH_RADIUS_M: f64 = 6_378_137.0;

/// Earth radius in micrometers (for UvoxID)
pub const EARTH_RADIUS_UM: f64 = EARTH_RADIUS_M * 1_000_000.0;

/// Astronomical Unit (AU) in meters
pub const AU_M: f64 = 149_597_870_700.0;

/// AU in micrometers (for UvoxID conversion)
pub const AU_UM: f64 = AU_M * 1_000_000.0;

// ---------------------------------------------------------------------------
// Sun orbital constants (low-precision / Meeus chapter 25)
// ---------------------------------------------------------------------------

/// Mean longitude of Sun at J2000 (deg)
pub const SUN_L0: f64 = 280.46646;

/// Mean anomaly of Sun at J2000 (deg)
pub const SUN_M0: f64 = 357.52911;

/// Rate of mean anomaly (deg per century)
pub const SUN_M_RATE: f64 = 35999.05029;

/// Eccentricity of Earth's orbit at J2000
pub const SUN_E0: f64 = 0.016708634;

// ---------------------------------------------------------------------------
// Moon orbital constants (simplified)
// ---------------------------------------------------------------------------

/// Mean longitude of Moon at J2000 (deg)
pub const MOON_L0: f64 = 218.316;

/// Mean anomaly of Moon at J2000 (deg)
pub const MOON_M0: f64 = 134.963;

/// Mean elongation at J2000 (deg)
pub const MOON_D0: f64 = 297.850;

/// Moon distance at mean perigee (km)
pub const MOON_PERIGEE_KM: f64 = 363_300.0;

/// Moon distance at mean apogee (km)
pub const MOON_APOGEE_KM: f64 = 405_500.0;

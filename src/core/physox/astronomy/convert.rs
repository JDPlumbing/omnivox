//! Conversion utilities between:
//!   - UvoxId (r_um, lat_code, lon_code)
//!   - spherical coordinates (radius, lat, lon)
//!   - Cartesian vectors for astronomy use
//!
//! These conversions are deterministic and reversible +
//! produce µm-level precision around Earth.

use crate::core::uvoxid::UvoxId;

/// Scale factors
const LATLON_SCALE: f64 = 1e11;           // ±90 degrees → ±90e11 int
const EARTH_RADIUS_M: f64 = 6_371_000.0;  // for geocode helpers

/// Simple 3D vector for astronomy calculations.
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// ------------------------------------------------------------
/// UvoxId → spherical angles (in radians)
/// ------------------------------------------------------------
pub fn uvox_to_spherical(id: &UvoxId) -> (f64, f64, f64) {
    let r = id.r_um as f64 * 1e-6; // micrometers → meters
    let lat = (id.lat_code as f64) / LATLON_SCALE; // degrees
    let lon = (id.lon_code as f64) / LATLON_SCALE; // degrees

    let lat_rad = lat.to_radians();
    let lon_rad = lon.to_radians();

    (r, lat_rad, lon_rad)
}

/// ------------------------------------------------------------
/// Spherical → UvoxId
/// lat/lon in degrees
/// r in meters
/// ------------------------------------------------------------
pub fn spherical_to_uvox(r_m: f64, lat_deg: f64, lon_deg: f64) -> UvoxId {
    let r_um = (r_m * 1e6).round() as i64;

    let lat_code = (lat_deg * LATLON_SCALE).round() as i64;
    let lon_code = (lon_deg * LATLON_SCALE).round() as i64;

    UvoxId {
        r_um,
        lat_code,
        lon_code,
    }
}

/// ------------------------------------------------------------
/// UvoxId → Cartesian (for astronomy)
/// ------------------------------------------------------------
pub fn uvox_to_cartesian(id: &UvoxId) -> Vec3 {
    let (r_m, lat, lon) = uvox_to_spherical(id);

    let cos_lat = lat.cos();
    Vec3 {
        x: r_m * cos_lat * lon.cos(),
        y: r_m * cos_lat * lon.sin(),
        z: r_m * lat.sin(),
    }
}

/// ------------------------------------------------------------
/// Cartesian → UvoxId
/// ------------------------------------------------------------
pub fn cartesian_to_uvox(v: Vec3) -> UvoxId {
    let r = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();

    let lat = (v.z / r).asin();             // radians
    let lon = v.y.atan2(v.x);               // radians

    spherical_to_uvox(
        r,
        lat.to_degrees(),
        lon.to_degrees(),
    )
}

/// ------------------------------------------------------------
/// Helper: build UvoxId from latitude/longitude/elevation
/// ------------------------------------------------------------
pub fn from_lat_lon(lat_deg: f64, lon_deg: f64, elevation_m: f64) -> UvoxId {
    spherical_to_uvox(
        EARTH_RADIUS_M + elevation_m,
        lat_deg,
        lon_deg,
    )
}

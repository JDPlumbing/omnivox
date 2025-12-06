//! Conversion utilities between:
//!   - UvoxId (r_um, lat_code, lon_code)
//!   - spherical coordinates (radius, lat, lon)
//!   - Cartesian vectors for astronomy use

use crate::core::uvoxid::{UvoxId, RUm, LatCode, LonCode};

/// Scale factors
const LATLON_SCALE: f64 = 1e11;
const EARTH_RADIUS_M: f64 = 6_371_000.0;

/// Simple 3D vector for astronomy calculations.
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// ------------------------------------------------------------
/// UvoxId → spherical (r meters, lat rad, lon rad)
/// ------------------------------------------------------------
pub fn uvox_to_spherical(id: &UvoxId) -> (f64, f64, f64) {
    let r_m = id.r_um.meters();              // µm → m
    let lat_deg = id.lat_code.degrees();     // wrapper → f64 degrees
    let lon_deg = id.lon_code.degrees();

    (
        r_m,
        lat_deg.to_radians(),
        lon_deg.to_radians(),
    )
}

/// ------------------------------------------------------------
/// Spherical → UvoxId
/// lat/lon in degrees, r in meters
/// ------------------------------------------------------------
pub fn spherical_to_uvox(r_m: f64, lat_deg: f64, lon_deg: f64) -> UvoxId {
    let r_um = RUm((r_m * 1e6).round() as i64);
    let lat_code = LatCode((lat_deg * LATLON_SCALE).round() as i64);
    let lon_code = LonCode((lon_deg * LATLON_SCALE).round() as i64);

    UvoxId::new(r_um, lat_code, lon_code)
}

/// ------------------------------------------------------------
/// UvoxId → Cartesian
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

    let lat = (v.z / r).asin();
    let lon = v.y.atan2(v.x);

    spherical_to_uvox(
        r,
        lat.to_degrees(),
        lon.to_degrees(),
    )
}

/// ------------------------------------------------------------
/// Helper: latitude/longitude/elevation → UvoxId
/// ------------------------------------------------------------
pub fn from_lat_lon(lat_deg: f64, lon_deg: f64, elevation_m: f64) -> UvoxId {
    spherical_to_uvox(
        EARTH_RADIUS_M + elevation_m,
        lat_deg,
        lon_deg,
    )
}

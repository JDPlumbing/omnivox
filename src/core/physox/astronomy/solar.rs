//! Solar ephemeris for Omnivox
//!
//! - Uses SimTime → Julian Date
//! - Returns Sun position in Earth-centered inertial frame (ECI)
//! - Then maps to UvoxId via astronomy/convert.rs

use crate::core::tdt::sim_time::SimTime;
//use crate::core::physox::astronomy::julian::simtime_to_julian;
use crate::core::physox::astronomy::convert::{Vec3, cartesian_to_uvox};
use crate::core::uvoxid::UvoxId;
use std::f64::consts::PI;

pub struct SunPosition {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub dist_km: f64,
    pub vec_m: Vec3,
    pub uvox: UvoxId,
}

pub fn solar_position(t: SimTime) -> SunPosition {
    let (ra, dec, r_au) = solar_ra_dec(t);
    let vec = solar_vector(t);

    let dist_km = r_au * 149_597_870.7; // km

    SunPosition {
        ra_deg: ra,
        dec_deg: dec,
        dist_km,
        vec_m: vec,
        uvox: cartesian_to_uvox(vec),
    }
}

/// Degrees → radians
#[inline]
fn deg2rad(x: f64) -> f64 { x * PI / 180.0 }

/// Radians → degrees
#[inline]
fn rad2deg(x: f64) -> f64 { x * 180.0 / PI }

/// Normalize angle in degrees
#[inline]
fn norm360(x: f64) -> f64 {
    let mut y = x % 360.0;
    if y < 0.0 { y += 360.0; }
    y
}

/// ------------------------------------------------------------
/// Compute solar RA/Dec + distance (AU)
/// Meeus "low precision" (error < 1′)
/// ------------------------------------------------------------
pub fn solar_ra_dec(t: SimTime) -> (f64, f64, f64) {
    let (_, jd) = crate::core::physox::astronomy::julian::simtime_to_julian(t);


    let n = jd - 2451545.0;      // days since J2000
    let g = deg2rad(norm360(357.529 + 0.98560028 * n)); // mean anomaly (rad)

    // Ecliptic longitude
    let lambda = deg2rad(
        norm360(280.459 + 0.98564736 * n + 1.915 * g.sin() + 0.020 * (2.0*g).sin())
    );

    // Obliquity of the ecliptic
    let eps = deg2rad(23.439 - 0.00000036 * n);

    // Distance (AU)
    let r = 1.00014 - 0.01671 * g.cos() - 0.00014 * (2.0*g).cos();

    // Convert to RA/Dec
    let sin_lambda = lambda.sin();
    let cos_lambda = lambda.cos();
    let sin_eps = eps.sin();
    let cos_eps = eps.cos();

    let alpha = rad2deg((sin_lambda * cos_eps).atan2(cos_lambda)); // RA deg
    let delta = rad2deg((sin_lambda * sin_eps).asin());            // Dec deg

    (alpha, delta, r)
}

/// ------------------------------------------------------------
/// Convert RA/Dec/distance → Cartesian ECI vector
/// ------------------------------------------------------------
pub fn solar_vector(t: SimTime) -> Vec3 {
    let (ra_deg, dec_deg, r_au) = solar_ra_dec(t);

    let ra = deg2rad(ra_deg);
    let dec = deg2rad(dec_deg);

    // 1 AU = 1.496e11 meters
    let distance_m = r_au * 1.496e11;

    let cos_dec = dec.cos();

    Vec3 {
        x: distance_m * cos_dec * ra.cos(),
        y: distance_m * cos_dec * ra.sin(),
        z: distance_m * dec.sin(),
    }
}

/// ------------------------------------------------------------
/// Convert Sun → UvoxId
/// ------------------------------------------------------------
///
/// WARNING:
/// - This is geocentric position (Earth-centered)
/// - lat/lon are meaningless; only distance + direction
///
/// For sky position at a specific Earth location,
/// you apply topocentric corrections (phase 2).
pub fn solar_uvox(t: SimTime) -> UvoxId {
    let vec = solar_vector(t);
    cartesian_to_uvox(vec)
}


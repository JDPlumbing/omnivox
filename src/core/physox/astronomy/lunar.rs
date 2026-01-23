//! Lunar ephemeris (low precision)
//!
//! Based on Jean Meeus – Astronomical Algorithms (chapters 47–49).
//!
//! Output:
//!   - Geocentric RA/Dec (°)
//!   - Distance (km)
//!   - Geocentric Cartesian ECI vector
//!   - UvoxId position (Earth-centered)

use crate::core::tdt::sim_time::SimTime;
//use crate::core::physox::astronomy::julian::simtime_to_julian;
use crate::core::physox::astronomy::convert::{Vec3, cartesian_to_uvox};
use crate::core::uvoxid::UvoxId;

use std::f64::consts::PI;

#[inline] fn deg2rad(x: f64) -> f64 { x * PI / 180.0 }
#[inline] fn rad2deg(x: f64) -> f64 { x * 180.0 / PI }

#[inline]
fn norm360(x: f64) -> f64 {
    let mut y = x % 360.0;
    if y < 0.0 { y += 360.0; }
    y
}

/// ------------------------------------------------------------
/// Low-precision Moon position
/// Meeus algorithms 47–49
/// ------------------------------------------------------------
pub fn lunar_ra_dec(t: SimTime) -> (f64, f64, f64) {
    let (_, jd) = crate::core::physox::astronomy::julian::simtime_to_julian(t);


    let d = jd - 2451545.0;       // days since J2000
    let n = d / 36525.0;          // Julian centuries

    // Mean longitude of Moon
    let l0 = norm360(218.316 + 481267.881 * n);

    // Mean elongation
    let d_m = norm360(297.850 + 445267.111 * n);

    // Sun mean anomaly
    let m = norm360(357.529 + 35999.050 * n);

    // Moon mean anomaly
    let m_m = norm360(134.963 + 477198.867 * n);

    // Moon argument of latitude
    let f = norm360(93.272 + 483202.017 * n);

    // Longitude corrections (low precision)
    let lon = l0
        + 6.289 * deg2rad(m_m).sin()
        + 1.274 * deg2rad(2.0 * d_m - m_m).sin()
        + 0.658 * deg2rad(2.0 * d_m).sin()
        + 0.214 * deg2rad(2.0 * m_m).sin()
        + 0.110 * deg2rad(d_m).sin();

    // Latitude correction
    let lat = 5.128 * deg2rad(f).sin()
            + 0.280 * deg2rad(m_m + f).sin()
            + 0.277 * deg2rad(m_m - f).sin()
            + 0.173 * deg2rad(2.0 * d_m - f).sin();

    // Distance in km (very rough)
    let dist_km =
        385001.0
        - 20905.0 * deg2rad(m_m).cos()
        - 3699.0 * deg2rad(2.0 * d_m - m_m).cos()
        - 2956.0 * deg2rad(2.0 * d_m).cos()
        - 570.0  * deg2rad(2.0 * m_m).cos();

    // Obliquity of ecliptic
    let eps = deg2rad(23.439 - 0.0000004 * d);

    let lon_r = deg2rad(lon);
    let lat_r = deg2rad(lat);

    let sin_lon = lon_r.sin();
    let cos_lon = lon_r.cos();
    let sin_lat = lat_r.sin();
    let cos_lat = lat_r.cos();
    let sin_eps = eps.sin();
    let cos_eps = eps.cos();

    // RA/Dec conversion
    let ra = rad2deg((sin_lon * cos_eps - tan(lat_r) * sin_eps).atan2(cos_lon));
    let dec = rad2deg((sin_lat * cos_eps + cos_lat * sin_eps * sin_lon).asin());

    (norm360(ra), dec, dist_km)
}

fn tan(x: f64) -> f64 { x.tan() }

/// ------------------------------------------------------------
/// Convert lunar RA/Dec + distance (km) → Cartesian ECI vector
/// ------------------------------------------------------------
pub fn lunar_vector(t: SimTime) -> Vec3 {
    let (ra_deg, dec_deg, dist_km) = lunar_ra_dec(t);

    let ra = deg2rad(ra_deg);
    let dec = deg2rad(dec_deg);

    let r_m = dist_km * 1000.0; // km → meters

    let cos_dec = dec.cos();

    Vec3 {
        x: r_m * cos_dec * ra.cos(),
        y: r_m * cos_dec * ra.sin(),
        z: r_m * dec.sin(),
    }
}

/// ------------------------------------------------------------
/// Convert Moon → UvoxId (geocentric)
/// ------------------------------------------------------------
pub fn lunar_uvox(t: SimTime) -> UvoxId {
    let vec = lunar_vector(t);
    cartesian_to_uvox(vec)
}

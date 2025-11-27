//! Topocentric solar position (alt/az) using RA/Dec + sidereal time.
//!
//! Computes:
//!   - solar elevation
//!   - solar azimuth
//!   - hour angle
//!   - zenith angle
//!   - irradiance cosine factor
//!   - subsolar point (point on Earth where sun is at zenith)

use std::f64::consts::PI;

use crate::core::uvoxid::UvoxId;
use crate::core::tdt::sim_time::SimTime;
use crate::core::physox::astronomy::solar::solar_ra_dec;
use crate::core::physox::astronomy::sidereal::lst_deg;
use crate::core::physox::astronomy::julian::simtime_to_julian;

#[inline]
fn deg2rad(x: f64) -> f64 { x * PI / 180.0 }
#[inline]
fn rad2deg(x: f64) -> f64 { x * 180.0 / PI }
#[inline]
fn norm360(x: f64) -> f64 {
    let mut y = x % 360.0;
    if y < 0.0 { y += 360.0; }
    y
}

#[derive(Debug, Clone)]
pub struct SunTopoResult {
    pub elevation_deg: f64,
    pub azimuth_deg: f64,
    pub zenith_deg: f64,
    pub hour_angle_deg: f64,
    pub subsolar: UvoxId,
    pub irradiance_factor: f64,
    pub is_daylight: bool,
}

pub fn sun_topocentric(observer: UvoxId, t: SimTime) -> SunTopoResult {

    // 1) Solar geocentric RA/Dec
    let (ra_deg, dec_deg, _dist_au) = solar_ra_dec(t);

    let ra = deg2rad(ra_deg);
    let dec = deg2rad(dec_deg);

    // 2) Observer position (spherical)
    let lat_deg = observer.lat_code as f64 / 1e11;
    let lon_deg = observer.lon_code as f64 / 1e11;

    let lat = deg2rad(lat_deg);

    // 3) Sidereal time (get only JD from tuple)
    let (_, jd) = simtime_to_julian(t);
    let lst = lst_deg(jd, lon_deg);        // Local sidereal time in degrees
    let hour_angle_deg = norm360(lst - ra_deg);
    let hour_angle = deg2rad(hour_angle_deg);

    // 4) Elevation angle
    let sin_alt =
        dec.sin() * lat.sin() +
        dec.cos() * lat.cos() * hour_angle.cos();

    let elevation_rad = sin_alt.asin();
    let elevation_deg = rad2deg(elevation_rad);

    let zenith_deg = 90.0 - elevation_deg;

    // 5) Azimuth
    let y = hour_angle.sin();
    let x = hour_angle.cos() * lat.sin() - dec.tan() * lat.cos();
    let mut azimuth_deg = rad2deg(y.atan2(x));

    // Convert to [0, 360)
    azimuth_deg = norm360(azimuth_deg);

    // 6) Subsolar point (lat = dec, lon = LST − RA − 180°)
    let subsolar_lat = dec_deg;
    let subsolar_lon = norm360(lst - ra_deg) - 180.0;

    let subsolar = UvoxId {
        r_um: observer.r_um,
        lat_code: (subsolar_lat * 1e11).round() as i64,
        lon_code: (subsolar_lon * 1e11).round() as i64,
    };

    // 7) Irradiance
    let irradiance_factor = sin_alt.max(0.0);
    let is_daylight = irradiance_factor > 0.0;

    SunTopoResult {
        elevation_deg,
        azimuth_deg,
        zenith_deg,
        hour_angle_deg,
        subsolar,
        irradiance_factor,
        is_daylight,
    }
}

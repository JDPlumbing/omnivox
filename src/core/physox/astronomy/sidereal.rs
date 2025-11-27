//! Sidereal time utilities for astronomical calculations.
//!
//! Provides:
//!   - GMST = Greenwich Mean Sidereal Time
//!   - GAST = Greenwich Apparent Sidereal Time
//!   - LST  = Local Sidereal Time, given observer longitude
//!
//! Accuracy: ~0.1 seconds (good enough for all sun/moon work)
//!
//! Based on:
//!   - Meeus, *Astronomical Algorithms*
//!   - IAU 2006/2000 resolutions (simplified)

use std::f64::consts::PI;
use crate::core::tdt::sim_time::SimTime;
use crate::core::physox::astronomy::julian::simtime_to_julian;

/// Normalize an angle in degrees to [0, 360)
#[inline]
fn norm360(mut x: f64) -> f64 {
    x %= 360.0;
    if x < 0.0 { x += 360.0; }
    x
}

/// Convert degrees → radians
#[inline]
fn deg2rad(x: f64) -> f64 { x * PI / 180.0 }

/// Convert radians → degrees
#[inline]
fn rad2deg(x: f64) -> f64 { x * 180.0 / PI }

/// ------------------------------------------------------------
/// Greenwich Mean Sidereal Time (GMST)
/// ------------------------------------------------------------
///
/// Formula from Meeus 2nd ed., Chapter 12.
///
/// Returns GMST in **degrees**, normalized to [0, 360).
pub fn gmst_deg(jd: f64) -> f64 {
    // Julian centuries since J2000
    let t = (jd - 2451545.0) / 36525.0;

    // GMST at 0h UT (in seconds)
    let gmst_sec =
        67310.54841
        + (876600.0 * 3600.0 + 8640184.812866) * t
        + 0.093104 * t*t
        - 6.2e-6 * t*t*t;

    // Convert seconds → degrees
    let gmst_deg = gmst_sec / 240.0;

    norm360(gmst_deg)
}

/// ------------------------------------------------------------
/// Equation of the Equinoxes (very small correction)
/// ------------------------------------------------------------
///
/// Needed to convert GMST → GAST.
/// Returns correction in **degrees**.
fn equation_of_equinoxes_deg(jd: f64) -> f64 {
    // Julian centuries
    let t = (jd - 2451545.0) / 36525.0;

    // Mean obliquity of ecliptic (arcsec)
    let eps0_arcsec =
        84381.448
        - 46.8150 * t
        - 0.00059 * t*t
        + 0.001813 * t*t*t;
    let eps0 = deg2rad(eps0_arcsec / 3600.0);

    // Longitude of ascending node (degrees)
    let omega = deg2rad(125.04452 - 1934.136261 * t);

    // Nutation in longitude (arcsec)
    let dpsi_arcsec =
        -17.20 * omega.sin()
        - 1.32 * (2.0 * omega).sin()
        + 0.23 * (3.0 * omega).sin()
        + 0.21 * (omega).sin(); // simplified series

    let dpsi = deg2rad(dpsi_arcsec / 3600.0);

    // Equation of Equinoxes = Δψ * cos(ε)
    rad2deg(dpsi * eps0.cos())
}

/// ------------------------------------------------------------
/// Greenwich Apparent Sidereal Time (GAST)
/// ------------------------------------------------------------
///
/// GAST = GMST + Equation of Equinoxes
/// Result in **degrees**.
pub fn gast_deg(jd: f64) -> f64 {
    let gmst = gmst_deg(jd);
    let eqeq = equation_of_equinoxes_deg(jd);
    norm360(gmst + eqeq)
}

/// ------------------------------------------------------------
/// Local Sidereal Time (LST)
/// ------------------------------------------------------------
///
/// Longitude in **degrees**:
///   - East positive  
///   - West negative  
///
/// Returns LST in **degrees**.
pub fn lst_deg(jd: f64, longitude_deg: f64) -> f64 {
    let gast = gast_deg(jd);
    norm360(gast + longitude_deg)
}

/// ------------------------------------------------------------
/// Helper: compute LST from a SimTime + longitude
/// ------------------------------------------------------------
pub fn lst_from_simtime(t: SimTime, longitude_deg: f64) -> f64 {
    let (_, jd) = simtime_to_julian(t);
    lst_deg(jd, longitude_deg)
}

/// ------------------------------------------------------------
/// UNIT TESTS
/// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gmst_2000() {
        // JD = 2451545.0 (J2000 at 12:00)
        let gmst = gmst_deg(2451545.0);
        // Known reference ~ 280.460618 degrees
        assert!((gmst - 280.46).abs() < 0.5);
    }

    #[test]
    fn test_lst_simple() {
        // J2000, longitude = 0
        let lst = lst_deg(2451545.0, 0.0);
        // Should be same as GMST
        let gmst = gmst_deg(2451545.0);
        assert!((lst - gmst).abs() < 1e-6);
    }
}

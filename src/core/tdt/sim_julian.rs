//! Julian date conversion for SimTime.
//!
//! Formula from US Naval Observatory / Meeus.

use crate::core::tdt::sim_time::SimTime;
use chrono::{Datelike, Timelike};

/*
/// Convert a SimTime (ns since Unix epoch) to Julian Day Number (JDN)
/// and Julian Date (JD with fractional day).
///
/// - JDN is integer day count starting at noon UTC Jan 1 4713 BC.
/// - JD is fractional.
/// - Returned JD is *UTC based*, fine for solar/lunar ephemeris.
pub fn simtime_to_julian(t: SimTime) -> (f64, f64) {

    // 1) Convert SimTime → chrono DateTime<Utc>
    let dt = t.to_datetime();

    // Extract Y M D H M S
    let year  = dt.year();
    let month = dt.month() as i32;
    let day   = dt.day() as i32;

    let hour   = dt.hour() as f64;
    let minute = dt.minute() as f64;
    let second = dt.second() as f64;
    let nanos  = dt.nanosecond() as f64;

    // Fractional day
    let frac_day = (hour + (minute + (second + nanos / 1e9) / 60.0) / 60.0) / 24.0;

    // Meeus month/year adjustment
    let (y, m) = if month <= 2 {
        (year - 1, month + 12)
    } else {
        (year, month)
    };

    // Gregorian calendar correction
    let a = (y as f64 / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();

    // Julian Day Number
    let jdn = (365.25 * (y as f64 + 4716.0)).floor()
        + (30.6001 * (m as f64 + 1.0)).floor()
        + (day as f64) + b - 1524.5;

    let jd = jdn + frac_day;

    (jdn, jd)
}
*/
/// Convert SimTime → Julian Date (JD)
/// Continuous UTC-based time, suitable for astronomy.
pub fn simtime_to_julian(t: SimTime) -> f64 {
    let dt = t.to_datetime();

    let year  = dt.year();
    let month = dt.month() as i32;
    let day   = dt.day() as i32;

    let hour   = dt.hour() as f64;
    let minute = dt.minute() as f64;
    let second = dt.second() as f64;
    let nanos  = dt.nanosecond() as f64;

    let frac_day =
        (hour + (minute + (second + nanos / 1e9) / 60.0) / 60.0) / 24.0;

    let (y, m) = if month <= 2 {
        (year - 1, month + 12)
    } else {
        (year, month)
    };

    let a = (y as f64 / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();

    let jd = (365.25 * (y as f64 + 4716.0)).floor()
        + (30.6001 * (m as f64 + 1.0)).floor()
        + (day as f64)
        + frac_day
        + b
        - 1524.5;

    jd
}

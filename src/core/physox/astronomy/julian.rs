//! Julian date conversion utilities for astronomy.
//!
//! Provides:
//!   - SimTime → Julian Date (JD)
//!   - JD → SimTime
//!   - JD century offset T = (JD - 2451545.0) / 36525.0

use chrono::{TimeZone, Utc, Datelike, Timelike};
use crate::core::tdt::sim_time::SimTime;

/// Convert SimTime → (JDN, JD)
pub fn simtime_to_julian(t: SimTime) -> (f64, f64) {
    let dt = t.to_datetime();

    let year  = dt.year();
    let month = dt.month() as i32;
    let day   = dt.day() as i32;

    let hour   = dt.hour() as f64;
    let minute = dt.minute() as f64;
    let second = dt.second() as f64;
    let nanos  = dt.nanosecond() as f64;

    let frac_day =
        (hour + (minute / 60.0) + (second / 3600.0) + (nanos / 3.6e12))
        / 24.0;

    let (y, m) = if month <= 2 {
        (year - 1, month + 12)
    } else {
        (year, month)
    };

    let a = (y as f64 / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();

    let jdn =
        (365.25 * (y as f64 + 4716.0)).floor()
        + (30.6001 * (m as f64 + 1.0)).floor()
        + (day as f64) + b - 1524.5;

    let jd = jdn + frac_day;

    (jdn, jd)
}


/// Convert Julian Date → SimTime (UTC)
pub fn julian_to_simtime(jd: f64) -> SimTime {
    let jd_adj = jd + 0.5;
    let z = jd_adj.floor();
    let f = jd_adj - z;

    let mut a = z;
    if z >= 2299161.0 {
        let alpha = ((z - 1867216.25) / 36524.25).floor();
        a = z + 1.0 + alpha - (alpha / 4.0).floor();
    }

    let b = a + 1524.0;
    let c = ((b - 122.1) / 365.25).floor();
    let d = (365.25 * c).floor();
    let e = ((b - d) / 30.6001).floor();

    let day = b - d - (30.6001 * e).floor() + f;
    let month = if e < 14.0 { e - 1.0 } else { e - 13.0 };
    let year = if month > 2.0 { c - 4716.0 } else { c - 4715.0 };

    let day_i = day.floor() as u32;
    let frac = day - day.floor();

    let total_seconds = frac * 86400.0;

    let hour   = (total_seconds / 3600.0).floor() as u32;
    let minute = ((total_seconds % 3600.0) / 60.0).floor() as u32;
    let second = (total_seconds % 60.0).floor() as u32;
    let nanos  = ((total_seconds % 1.0) * 1e9) as u32;

    let dt = Utc
        .with_ymd_and_hms(year as i32, month as u32, day_i,
                          hour, minute, second)
        .unwrap()
        .with_nanosecond(nanos)
        .unwrap();

    SimTime::from_datetime(dt)
}

pub fn julian_centuries(jd: f64) -> f64 {
    (jd - 2451545.0) / 36525.0
}

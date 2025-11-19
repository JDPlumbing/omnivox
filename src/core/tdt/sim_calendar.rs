//! Fixed, deterministic simulation calendar units.
//!
//! All units are expressed as exact integer multiples of nanoseconds.
//! These values DO NOT attempt to model astronomical irregularities
//! (leap years, leap seconds, axial drift, etc).
//!
//! They exist to make the simulation 100% deterministic, stable, and easy
//! to integrate over long time spans.
pub const NANOS_PER_MILLISECOND: i128 = 1_000_000;
pub const NANOS_PER_MICROSECOND: i128 = 1_000;
pub const NANOS_PER_SECOND: i128 = 1_000_000_000;
pub const NANOS_PER_MINUTE: i128 = 60 * NANOS_PER_SECOND;
pub const NANOS_PER_HOUR:   i128 = 60 * NANOS_PER_MINUTE;
pub const NANOS_PER_DAY:    i128 = 24 * NANOS_PER_HOUR;


/// Fixed "simulation year":
/// 365.25 days — engineering standard.
/// Converted through f64 but stored as exact i128.
pub const NANOS_PER_YEAR: i128 =
    (365.25_f64 * NANOS_PER_DAY as f64) as i128;

/// Fixed simulation month = 1/12 of a year.
/// This is NOT tied to real Gregorian months — purely deterministic.
pub const NANOS_PER_MONTH: i128 = NANOS_PER_YEAR / 12;

/// Fixed simulation week = 7 days.
pub const NANOS_PER_WEEK: i128 = 7 * NANOS_PER_DAY;

// ----------------------------------------------------------------------
// Helper functions
// ----------------------------------------------------------------------

/// Convert integer quantities into ns durations.
pub fn seconds(n: i64) -> i128 {
    n as i128 * NANOS_PER_SECOND
}

pub fn minutes(n: i64) -> i128 {
    n as i128 * NANOS_PER_MINUTE
}

pub fn hours(n: i64) -> i128 {
    n as i128 * NANOS_PER_HOUR
}

pub fn days(n: i64) -> i128 {
    n as i128 * NANOS_PER_DAY
}

pub fn weeks(n: i64) -> i128 {
    n as i128 * NANOS_PER_WEEK
}

pub fn months(n: i64) -> i128 {
    n as i128 * NANOS_PER_MONTH
}

pub fn years(n: i64) -> i128 {
    n as i128 * NANOS_PER_YEAR
}

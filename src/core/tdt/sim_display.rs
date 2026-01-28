// src/core/tdt/sim_display.rs
//
// Human and machine-friendly formatting helpers for SimTime,
// using the deterministic fixed simulation calendar.

use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_date::SimDate;
use crate::core::tdt::sim_calendar::*;
use crate::core::chronovox::ChronoEvent;
use serde::{Serialize, Deserialize};
use chrono::{SecondsFormat};
use serde_json::json;

/// ---------------------------------------------------------------------------
/// Time formatting selection
/// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TimeFormat {
    RawNs,      // just i128 (t.as_ns())
    Rfc3339,    // real date (simtime mapped to Unix epoch)
    SimDate,    // YYYY-MM-DD (deterministic sim calendar)
    SimMonth,   // zero-based synthetic month index
    SimWeek,    // zero-based synthetic week index
    SimDay,     // zero-based synthetic day index
}

/// ---------------------------------------------------------------------------
/// Core formatting entry point
/// ---------------------------------------------------------------------------
pub fn format_simtime(t: SimTime, fmt: TimeFormat) -> String {
    match fmt {
        TimeFormat::RawNs => format!("{}", t.as_ns()),
        TimeFormat::Rfc3339 => t
            .to_datetime()
            .to_rfc3339_opts(SecondsFormat::Millis, true),

        TimeFormat::SimDate => format_simdate(t),
        TimeFormat::SimMonth => sim_month_index(t).to_string(),
        TimeFormat::SimWeek  => sim_week_index(t).to_string(),
        TimeFormat::SimDay   => sim_day_index(t).to_string(),
    }
}

/// ---------------------------------------------------------------------------
/// Deterministic sub-index helpers
/// ---------------------------------------------------------------------------

/// Zero-based simulation month index
pub fn sim_month_index(t: SimTime) -> i64 {
    (t.as_ns() / NANOS_PER_MONTH) as i64
}

/// Zero-based simulation week index
pub fn sim_week_index(t: SimTime) -> i64 {
    (t.as_ns() / NANOS_PER_WEEK) as i64
}

/// Zero-based simulation day index
pub fn sim_day_index(t: SimTime) -> i64 {
    (t.as_ns() / NANOS_PER_DAY) as i64
}

/// Zero-based simulation hour index
pub fn sim_hour_index(t: SimTime) -> i64 {
    (t.as_ns() / NANOS_PER_HOUR) as i64
}

/// Convert SimTime → deterministic YYYY-MM-DD (SimDate)
pub fn sim_date_from_simtime(t: SimTime) -> SimDate {
    t.to_sim_date()
}

/// Human-friendly "YYYY-MM-DD"
pub fn format_simdate(t: SimTime) -> String {
    let d = t.to_sim_date();
    format!("{:04}-{:02}-{:02}", d.year, d.month, d.day)
}

/// Human RFC3339 timestamp
pub fn rfc3339_from_simtime(t: SimTime) -> String {
    t.to_datetime()
        .to_rfc3339_opts(SecondsFormat::Millis, true)
}


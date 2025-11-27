// tdt/sim_duration.rs

use crate::core::tdt::sim_calendar::*;
use std::ops::{Add, Sub, Mul, Div};
use serde::{Serialize, Deserialize};
/// Duration in simulation time: nanoseconds as i128.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SimDuration(pub i128);

impl SimDuration {
    // -------------------------------------------------------------
    // Constructors
    // -------------------------------------------------------------

    #[inline]
    pub fn from_ns(ns: i128) -> Self {
        Self(ns)
    }

    #[inline]
    pub fn from_seconds(sec: i64) -> Self {
        Self((sec as i128) * NANOS_PER_SECOND)
    }

    #[inline]
    pub fn seconds(n: i64) -> Self {
        Self(seconds(n))
    }

    #[inline]
    pub fn minutes(n: i64) -> Self {
        Self(minutes(n))
    }

    #[inline]
    pub fn hours(n: i64) -> Self {
        Self(hours(n))
    }

    #[inline]
    pub fn days(n: i64) -> Self {
        Self(days(n))
    }

    #[inline]
    pub fn weeks(n: i64) -> Self {
        Self(weeks(n))
    }

    #[inline]
    pub fn months(n: i64) -> Self {
        Self(months(n))
    }

    #[inline]
    pub fn years(n: i64) -> Self {
        Self(years(n))
    }

    // -------------------------------------------------------------
    // Accessors
    // -------------------------------------------------------------

    #[inline]
    pub fn as_ns(&self) -> i128 {
        self.0
    }

    #[inline]
    pub fn seconds_f64(&self) -> f64 {
        (self.0 as f64) / 1e9
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub const ZERO: SimDuration = SimDuration(0);

    // -------------------------------------------------------------
    // Human-friendly formatting
    // -------------------------------------------------------------

    /// Best-effort human-readable duration (e.g. "2.5 hours", "30 seconds")
    pub fn to_string_human(&self) -> String {
        let ns = self.0;

        if ns == 0 {
            return "0 ns".into();
        }

        if ns.abs() < NANOS_PER_MICROSECOND {
            return format!("{} ns", ns);
        }

        // microseconds
        if ns.abs() < NANOS_PER_MILLISECOND {
            return format!("{:.3} µs", ns as f64 / 1e3);
        }

        // milliseconds
        if ns.abs() < NANOS_PER_SECOND {
            return format!("{:.3} ms", ns as f64 / 1e6);
        }

        // seconds
        if ns.abs() < NANOS_PER_MINUTE {
            return format!("{:.3} s", ns as f64 / 1e9);
        }

        // minutes
        if ns.abs() < NANOS_PER_HOUR {
            return format!("{:.3} min", ns as f64 / 6e10);
        }

        // hours
        if ns.abs() < NANOS_PER_DAY {
            return format!("{:.3} h", ns as f64 / 3.6e12);
        }

        // days
        if ns.abs() < NANOS_PER_WEEK {
            return format!("{:.3} days", ns as f64 / NANOS_PER_DAY as f64);
        }

        // weeks
        if ns.abs() < NANOS_PER_MONTH {
            return format!("{:.3} weeks", ns as f64 / NANOS_PER_WEEK as f64);
        }

        // months (sim-month = 1/12 year)
        if ns.abs() < NANOS_PER_YEAR {
            return format!("{:.3} months", ns as f64 / NANOS_PER_MONTH as f64);
        }

        // years
        format!("{:.3} years", ns as f64 / NANOS_PER_YEAR as f64)
    }
}

// -------------------------------------------------------------
// Arithmetic
// -------------------------------------------------------------

impl Add for SimDuration {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for SimDuration {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Mul<i128> for SimDuration {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i128) -> Self {
        Self(self.0 * rhs)
    }
}

impl Div<i128> for SimDuration {
    type Output = Self;
    #[inline]
    fn div(self, rhs: i128) -> Self {
        Self(self.0 / rhs)
    }
}

// Allow 64-bit multipliers too
impl Mul<i64> for SimDuration {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i64) -> Self {
        Self(self.0 * rhs as i128)
    }
}

impl Div<i64> for SimDuration {
    type Output = Self;
    #[inline]
    fn div(self, rhs: i64) -> Self {
        Self(self.0 / rhs as i128)
    }
}

// -------------------------------------------------------------
// Additional conversions
// -------------------------------------------------------------
impl SimDuration {
    pub fn as_seconds(&self) -> f64 {
        (self.0 as f64) / 1e9
    }
    pub fn as_secs_f64(&self) -> f64 {
        self.as_seconds()
    }
}

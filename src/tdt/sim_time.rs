use std::ops::{Add, Sub, AddAssign, SubAssign};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, TimeZone, Utc, Duration};

/// Number of nanoseconds since Unix epoch.
/// Range: ±1.7e38 ns → ±292,000,000,000 years.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SimTime(pub i128);

/// A duration in nanoseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SimDuration(pub i128);

impl SimTime {
    /// Create from nanoseconds.
    pub fn from_ns(ns: i128) -> Self {
        SimTime(ns)
    }

    /// Create from seconds.
    pub fn from_seconds(sec: i64) -> Self {
        SimTime(sec as i128 * 1_000_000_000)
    }

    /// Convert to nanoseconds.
    pub fn as_ns(&self) -> i128 {
        self.0
    }

    /// Convert to seconds (float).
    pub fn as_seconds_f64(&self) -> f64 {
        self.0 as f64 / 1_000_000_000.0
    }

    /// Create from chrono DateTime<Utc>.
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        let unix_ns = dt.timestamp_nanos_opt()
            .expect("Datetime out of range");

        SimTime(unix_ns as i128)
    }

    /// Convert to chrono DateTime<Utc>.
    pub fn to_datetime(&self) -> DateTime<Utc> {
        let sec = (self.0 / 1_000_000_000) as i64;
        let ns = (self.0 % 1_000_000_000) as u32;

        Utc.timestamp_opt(sec, ns).unwrap()
    }

    /// Convert to RFC3339 string.
    pub fn to_rfc3339(&self) -> String {
        self.to_datetime().to_rfc3339()
    }
}

impl SimDuration {
    pub fn from_ns(ns: i128) -> Self {
        SimDuration(ns)
    }

    pub fn from_seconds(sec: i64) -> Self {
        SimDuration(sec as i128 * 1_000_000_000)
    }

    pub fn as_ns(&self) -> i128 {
        self.0
    }

    pub fn as_seconds_f64(&self) -> f64 {
        self.0 as f64 / 1_000_000_000.0
    }
}

/// Time arithmetic
impl Add<SimDuration> for SimTime {
    type Output = SimTime;

    fn add(self, rhs: SimDuration) -> Self::Output {
        SimTime(self.0 + rhs.0)
    }
}

impl Sub<SimDuration> for SimTime {
    type Output = SimTime;

    fn sub(self, rhs: SimDuration) -> Self::Output {
        SimTime(self.0 - rhs.0)
    }
}

impl AddAssign<SimDuration> for SimTime {
    fn add_assign(&mut self, dur: SimDuration) {
        self.0 += dur.0;
    }
}

impl SubAssign<SimDuration> for SimTime {
    fn sub_assign(&mut self, dur: SimDuration) {
        self.0 -= dur.0;
    }
}

/// Difference between two times → a duration
impl Sub<SimTime> for SimTime {
    type Output = SimDuration;

    fn sub(self, rhs: SimTime) -> Self::Output {
        SimDuration(self.0 - rhs.0)
    }
}

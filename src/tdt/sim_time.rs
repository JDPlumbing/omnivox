// tdt/sim_time.rs
use chrono::{DateTime, TimeZone, Utc};

/// Absolute point in simulation time: nanoseconds since Unix epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimTime(pub i128);

impl SimTime {
    /// Construct from raw nanoseconds
    pub fn from_ns(ns: i128) -> Self {
        SimTime(ns)
    }

    /// Construct from whole seconds
    pub fn from_seconds(sec: i64) -> Self {
        SimTime((sec as i128) * 1_000_000_000)
    }

    /// Convert a wall clock datetime → SimTime
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        let ns = dt.timestamp_nanos_opt()
            .expect("datetime out of range") as i128;
        SimTime(ns)
    }

    /// Convert SimTime → chrono::DateTime<Utc>
    pub fn to_datetime(self) -> DateTime<Utc> {
        let sec = (self.0 / 1_000_000_000) as i64;
        let nsec = (self.0 % 1_000_000_000) as u32;
        Utc.timestamp_opt(sec, nsec).unwrap()
    }

    /// Add a SimDuration
    pub fn add(self, dt: super::sim_duration::SimDuration) -> Self {
        SimTime(self.0 + dt.0)
    }

    /// Subtract two SimTimes → SimDuration
    pub fn diff(self, other: SimTime) -> super::sim_duration::SimDuration {
        super::sim_duration::SimDuration(self.0 - other.0)
    }
}

use chrono::{DateTime, Duration, Utc};
use crate::tdt::core::TimeDelta;
use crate::tdt::sim_time::SimDuration;

#[derive(Debug, Clone)]
pub struct SimClock {
    pub start_ns: i128,
    pub end_ns: i128,
    pub current_ns: i128,
    pub step_ns: i128,
}

impl SimClock {
    /// Construct using human wall-clock dates
    pub fn from_wall_dates(start: DateTime<Utc>, end: DateTime<Utc>, step: Duration) -> Self {
        Self {
            start_ns: start.timestamp_nanos() as i128,
            end_ns: end.timestamp_nanos() as i128,
            current_ns: start.timestamp_nanos() as i128,
            step_ns: step.num_nanoseconds().unwrap() as i128,
        }
    }

    /// Advance simulation time by step_ns
    pub fn advance(&mut self) -> bool {
        if self.current_ns >= self.end_ns {
            return false;
        }
        self.current_ns += self.step_ns;
        true
    }

    /// Return time difference between start and current in TimeDelta
    pub fn now_delta(&self) -> TimeDelta {
        TimeDelta::from_sim_duration(SimDuration::from_ns(self.current_ns - self.start_ns))
    }

    /// Human-readable step duration
    pub fn step_seconds(&self) -> f64 {
        (self.step_ns as f64) / 1e9
    }

    /// Conversion helpers (f64 → nanoseconds)
    pub fn minutes_to_ns(min: f64) -> i128 {
        (min * 60.0 * 1e9) as i128
    }

    pub fn hours_to_ns(hours: f64) -> i128 {
        (hours * 3600.0 * 1e9) as i128
    }

    pub fn days_to_ns(days: f64) -> i128 {
        (days * 86400.0 * 1e9) as i128
    }

    pub fn weeks_to_ns(weeks: f64) -> i128 {
        (weeks * 604800.0 * 1e9) as i128
    }

    /// Convert current simtime to DateTime<Utc>
    pub fn current_datetime(&self) -> DateTime<Utc> {
        let secs = (self.current_ns / 1_000_000_000) as i64;
        let nanos = (self.current_ns % 1_000_000_000) as u32;

        DateTime::<Utc>::from_timestamp(secs, nanos).unwrap()
    }

    /// Fraction complete (0–1)
    pub fn progress(&self) -> f64 {
        let total = (self.end_ns - self.start_ns).max(1);
        let elapsed = (self.current_ns - self.start_ns).max(0);
        (elapsed as f64) / (total as f64)
    }
}

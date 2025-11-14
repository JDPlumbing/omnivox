use chrono::{DateTime, Duration, Utc};
use crate::tdt::sim_time::SimTime;
use crate::tdt::sim_duration::SimDuration;

#[derive(Debug, Clone)]
pub struct SimClock {
    pub start: SimTime,
    pub end: SimTime,
    pub current: SimTime,
    pub step: SimDuration,
}

impl SimClock {
    pub fn from_wall_dates(start: DateTime<Utc>, end: DateTime<Utc>, step: Duration) -> Self {
        let start_ns = start.timestamp_nanos_opt().unwrap() as i128;
        let end_ns = end.timestamp_nanos_opt().unwrap() as i128;
        let step_ns = step.num_nanoseconds().unwrap() as i128;

        SimClock {
            start: SimTime(start_ns),
            end: SimTime(end_ns),
            current: SimTime(start_ns),
            step: SimDuration(step_ns),
        }
    }

    pub fn advance(&mut self) -> Option<SimTime> {
        if self.current.0 >= self.end.0 {
            return None;
        }

        self.current = self.current.add(self.step);
        Some(self.current)
    }

    pub fn current_ns(&self) -> i128 {
        self.current.0
    }

    pub fn step_ns(&self) -> i128 {
        self.step.0
    }

    pub fn current_wall_time(&self) -> DateTime<Utc> {
        self.current.to_datetime()
    }

    pub fn step_seconds(&self) -> f64 {
        (self.step.0 as f64) / 1_000_000_000.0
    }
}

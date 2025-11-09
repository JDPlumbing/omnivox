use chrono::{DateTime, Duration, Utc};
use crate::tdt::core::TimeDelta;

#[derive(Debug, Clone)]
pub struct SimClock {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub current: DateTime<Utc>,
    pub step: Duration,
}

impl SimClock {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>, step: Duration) -> Self {
        Self { start, end, current: start, step }
    }

    pub fn advance(&mut self) -> bool {
        if self.current >= self.end {
            return false;
        }
        self.current = self.current + self.step;
        true
    }

    pub fn now_delta(&self) -> TimeDelta {
        TimeDelta::between(self.start, self.current)
    }

    pub fn progress(&self) -> f64 {
        let total = self.end - self.start;
        let elapsed = self.current - self.start;
        elapsed.num_seconds() as f64 / total.num_seconds().max(1) as f64
    }
}

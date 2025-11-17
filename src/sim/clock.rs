use chrono::{DateTime, Duration, Utc};
use crate::tdt::sim_time::SimTime;
use crate::tdt::sim_duration::SimDuration;

#[derive(Debug, Clone)]
pub struct SimClock {
    /// Absolute simulation start time
    pub start: SimTime,

    /// Absolute simulation end time
    pub end: SimTime,

    /// Current simulation time
    pub current: SimTime,

    /// Fixed step size per tick
    pub step: SimDuration,
}

impl SimClock {

    // ------------------------------------------------------------
    // Constructors
    // ------------------------------------------------------------

    /// Create a clock from real-world datetimes + chrono::Duration step
    /// (useful for scratch files + quick simulation setups)
    pub fn from_wall_dates(start: DateTime<Utc>, end: DateTime<Utc>, step: Duration) -> Self {
        let step_ns = step.num_nanoseconds()
            .expect("step duration out of range")
            as i128;

        Self {
            start: SimTime::from_datetime(start),
            current: SimTime::from_datetime(start),
            end:   SimTime::from_datetime(end),
            step:  SimDuration(step_ns),
        }
    }

    /// Create a clock directly from nanosecond values
    /// (useful if simulation steps are precomputed)
    pub fn from_utc_range(start: DateTime<Utc>, end: DateTime<Utc>, step_ns: i128) -> Self {
        Self {
            start:   SimTime::from_datetime(start),
            current: SimTime::from_datetime(start),
            end:     SimTime::from_datetime(end),
            step:    SimDuration(step_ns),
        }
    }

    // ------------------------------------------------------------
    // Time accessors
    // ------------------------------------------------------------

    /// Get current absolute simulation time as ns since Unix epoch
    #[inline]
    pub fn current_ns(&self) -> i128 {
        self.current.as_ns()
    }

    /// Get step size in ns
    #[inline]
    pub fn step_ns(&self) -> i128 {
        self.step.as_ns()
    }

    /// Retrieve current wall-clock datetime representation
    #[inline]
    pub fn current_wall_time(&self) -> DateTime<Utc> {
        self.current.to_datetime()
    }

    /// Step size in seconds (float), useful for physics integration
    #[inline]
    pub fn step_seconds(&self) -> f64 {
        self.step.as_ns() as f64 / 1e9
    }

    // ------------------------------------------------------------
    // Time progression
    // ------------------------------------------------------------

    /// Advance by exactly one simulation step.
    /// Returns `true` if we advanced, `false` if simulation is finished.
    pub fn advance(&mut self) -> bool {
        if self.current.as_ns() >= self.end.as_ns() {
            return false;
        }

        self.current = self.current.add(self.step);
        true
    }

    /// Check whether simulation time has reached or passed the end.
    #[inline]
    pub fn is_finished(&self) -> bool {
        self.current.as_ns() >= self.end.as_ns()
    }
}

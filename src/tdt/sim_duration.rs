// tdt/sim_duration.rs
/// Duration in simulation time: nanoseconds as i128.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimDuration(pub i128);

impl SimDuration {
    pub fn from_ns(ns: i128) -> Self {
        Self(ns)
    }

    pub fn from_seconds(sec: i64) -> Self {
        Self((sec as i128) * 1_000_000_000)
    }

    pub fn seconds_f64(&self) -> f64 {
        (self.0 as f64) / 1e9
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn as_ns(&self) -> i128 {
        self.0
    }
}

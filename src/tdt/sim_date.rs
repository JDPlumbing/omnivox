use serde::{Serialize, Deserialize};
use std::fmt;
use crate::tdt::sim_time::SimTime;
use crate::tdt::sim_time;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimDate {
    pub year: i32,
    pub month: u8, // 1–12
    pub day: u8,   // 1–30-ish (fixed deterministic calendar)
}

impl SimDate {
    /// Safe constructor
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        assert!((1..=12).contains(&month), "month out of range");
        assert!((1..=31).contains(&day), "day out of range");
        SimDate { year, month, day }
    }

    /// Convert to SimTime using the fixed sim calendar
    pub fn to_sim_time(&self) -> sim_time::SimTime {
        SimTime::from_sim_date(*self)

    }
}

impl fmt::Display for SimDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

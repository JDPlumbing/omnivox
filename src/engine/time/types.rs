use crate::core::tdt::sim_display::TimeFormat;

#[derive(Debug, Clone)]
pub struct FormattedSimTime {
    pub formatted: String,
    pub format: TimeFormat,
    pub ns: i128,
}

#[derive(Debug, Clone)]
pub struct TimeDelta {
    pub delta_ns: i128,
    pub human: String,
}
// engine/time/types.rs
#[derive(Debug, Clone)]
pub struct HumanDuration {
    pub ns: i128,
    pub human: String,
}
#[derive(Debug, Clone)]
pub struct JulianDateResult {
    pub julian_date: f64,
    pub ns: i128,
}
// engine/time/types.rs
#[derive(Debug, Clone)]
pub struct SimDateResult {
    pub simdate: String,
    pub ns: i128,
}

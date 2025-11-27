use crate::core::tdt::sim_calendar::*;
use crate::core::tdt::sim_date::SimDate;
use chrono::{DateTime, Utc, TimeZone, SecondsFormat};
use serde::{Serialize, Deserialize, Serializer};
use crate::core::tdt::sim_duration::SimDuration;
//use crate::core::tdt::sim_julian::{simtime_to_julian, julian_to_simtime};
/// Absolute simulation time: nanoseconds since Unix epoch.
/// Deterministic, monotonic, integer-based.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SimTime(pub i128);

impl SimTime {

    // ----------------------------------------------------
    // Constructors
    // ----------------------------------------------------

    pub fn from_ns(ns: i128) -> Self {
        SimTime(ns)
    }

    pub fn from_seconds(sec: i64) -> Self {
        SimTime(sec as i128 * NANOS_PER_SECOND)
    }

    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        SimTime(dt.timestamp_nanos_opt().unwrap() as i128)
    }

    pub fn from_sim_date(date: SimDate) -> Self {
        let year_ns  = (date.year  as i128) * NANOS_PER_YEAR;
        let month_ns = (date.month as i128 - 1) * NANOS_PER_MONTH;
        let day_ns   = (date.day   as i128 - 1) * NANOS_PER_DAY;
        SimTime(year_ns + month_ns + day_ns)
    }

    // ----------------------------------------------------
    // Conversions
    // ----------------------------------------------------

    pub fn as_ns(&self) -> i128 {
        self.0
    }

    pub fn to_datetime(self) -> DateTime<Utc> {
        let secs = (self.0 / 1_000_000_000) as i64;
        let nanos = (self.0 % 1_000_000_000) as u32;
        Utc.timestamp_opt(secs, nanos).unwrap()
    }

    pub fn to_sim_date(&self) -> SimDate {
        let mut ns = self.0.max(0);

        let year = (ns / NANOS_PER_YEAR) as i32;
        ns %= NANOS_PER_YEAR;

        let month = (ns / NANOS_PER_MONTH) as u8 + 1;
        ns %= NANOS_PER_MONTH;

        let day = (ns / NANOS_PER_DAY) as u8 + 1;

        SimDate { year, month, day }
    }

    pub fn format_rfc3339(&self) -> String {
        self.to_datetime()
            .to_rfc3339_opts(SecondsFormat::Nanos, true)
    }

    // ----------------------------------------------------
    // Arithmetic
    // ----------------------------------------------------

    pub fn add_ns(self, ns: i128) -> Self {
        SimTime(self.0 + ns)
    }

    pub fn add_seconds(self, n: i64) -> Self {
        SimTime(self.0 + seconds(n))
    }

    pub fn add_minutes(self, n: i64) -> Self {
        SimTime(self.0 + minutes(n))
    }

    pub fn add_hours(self, n: i64) -> Self {
        SimTime(self.0 + hours(n))
    }

    pub fn add_days(self, n: i64) -> Self {
        SimTime(self.0 + days(n))
    }

    pub fn add_weeks(self, n: i64) -> Self {
        SimTime(self.0 + weeks(n))
    }

    pub fn add_months(self, n: i64) -> Self {
        SimTime(self.0 + months(n))
    }

    pub fn add_years(self, n: i64) -> Self {
        SimTime(self.0 + years(n))
    }

    #[inline]
    pub fn add(self, dt: SimDuration) -> Self {
        SimTime(self.0 + dt.0)
    }
    
    #[inline]
    pub fn ticks(&self, _unit: &str) -> i128 {
        self.0
    }


}

// ------------------------------------------------------------
// Serde helper
// ------------------------------------------------------------

pub fn serialize_simtime<S>(t: &SimTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let dt = t.to_datetime();
    s.serialize_str(&dt.to_rfc3339_opts(SecondsFormat::Nanos, true))
}
// ------------------------------------------------------------
// Now function
// ------------------------------------------------------------
impl SimTime {
    pub fn now() -> Self {
        Self::from_datetime(Utc::now())
    }
}
// ------------------------------------------------------------
// Julian Day conversions
// ------------------------------------------------------------
/*
impl SimTime {
    pub fn to_julian(&self) -> f64 {
        simtime_to_julian(*self)
    }
}

impl SimTime {
    pub fn from_julian(jd: f64) -> Self {
        julian_to_simtime(jd)
    }
}
*/
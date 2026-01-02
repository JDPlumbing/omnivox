use crate::core::tdt::sim_calendar::*;
use crate::core::tdt::sim_date::SimDate;
use chrono::{DateTime, Utc, TimeZone, SecondsFormat};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::Error;
use crate::core::tdt::sim_duration::SimDuration;
//use crate::core::tdt::sim_julian::{simtime_to_julian, julian_to_simtime};
/// Absolute simulation time: nanoseconds since Unix epoch.
/// Deterministic, monotonic, integer-based.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Hash)]
pub struct SimTime(pub i128);


impl<'de> Deserialize<'de> for SimTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let ns: i128 = s.parse().map_err(serde::de::Error::custom)?;
        Ok(SimTime(ns))
    }
}

/// Implementation of `SimTime` providing constructors, conversions, and arithmetic operations.
impl SimTime {

    // ----------------------------------------------------
    // Constructors
    // ----------------------------------------------------

    /// Creates a new `SimTime` from nanoseconds.
    ///
    /// # Arguments
    ///
    /// * `ns` - The number of nanoseconds since the epoch
    ///
    /// # Examples
    ///
    /// ```
    /// let time = SimTime::from_ns(1_000_000_000);
    /// ```
    pub fn from_ns(ns: i128) -> Self {
        SimTime(ns)
    }

    /// Creates a new `SimTime` from seconds.
    ///
    /// # Arguments
    ///
    /// * `sec` - The number of seconds since the epoch
    ///
    /// # Examples
    ///
    /// ```
    /// let time = SimTime::from_seconds(60);
    /// ```
    pub fn from_seconds(sec: i64) -> Self {
        SimTime(sec as i128 * NANOS_PER_SECOND)
    }

    /// Creates a new `SimTime` from a UTC datetime.
    ///
    /// # Arguments
    ///
    /// * `dt` - A `DateTime<Utc>` instance
    ///
    /// # Panics
    ///
    /// Panics if the datetime cannot be converted to nanoseconds.
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        SimTime(dt.timestamp_nanos_opt().unwrap() as i128)
    }

    /// Creates a new `SimTime` from a `SimDate`.
    ///
    /// # Arguments
    ///
    /// * `date` - A `SimDate` containing year, month, and day
    ///
    /// # Examples
    ///
    /// ```
    /// let date = SimDate { year: 2024, month: 1, day: 1 };
    /// let time = SimTime::from_sim_date(date);
    /// ```
    pub fn from_sim_date(date: SimDate) -> Self {
        let year_ns  = (date.year  as i128) * NANOS_PER_YEAR;
        let month_ns = (date.month as i128 - 1) * NANOS_PER_MONTH;
        let day_ns   = (date.day   as i128 - 1) * NANOS_PER_DAY;
        SimTime(year_ns + month_ns + day_ns)
    }

    // ----------------------------------------------------
    // Conversions
    // ----------------------------------------------------

    /// Returns the time as nanoseconds since the epoch.
    ///
    /// # Returns
    ///
    /// The number of nanoseconds as an `i128`
    pub fn as_ns(&self) -> i128 {
        self.0
    }

    /// Converts the `SimTime` to a UTC datetime.
    ///
    /// # Returns
    ///
    /// A `DateTime<Utc>` instance
    ///
    /// # Panics
    ///
    /// Panics if the timestamp is out of range for `DateTime<Utc>`.
    pub fn to_datetime(self) -> DateTime<Utc> {
        let secs = (self.0 / 1_000_000_000) as i64;
        let nanos = (self.0 % 1_000_000_000) as u32;
        Utc.timestamp_opt(secs, nanos).unwrap()
    }

    /// Converts the `SimTime` to a `SimDate`.
    ///
    /// This method supports negative time values and correctly handles
    /// negative years, months, and days by shifting the origin.
    ///
    /// # Returns
    ///
    /// A `SimDate` containing year, month (1-12), and day (1-31)
    pub fn to_sim_date(&self) -> SimDate {
        let mut ns = self.0; // ← DO NOT CLAMP TO ZERO

        // Allow negative time: shift the origin so division works
        // Convert negative values into (year, month, day)
        let mut year = ns / NANOS_PER_YEAR;
        ns %= NANOS_PER_YEAR;

        if ns < 0 {
            ns += NANOS_PER_YEAR;
            year -= 1;
        }

        let mut month = ns / NANOS_PER_MONTH;
        ns %= NANOS_PER_MONTH;

        if ns < 0 {
            ns += NANOS_PER_MONTH;
            month -= 1;
        }

        let day = ns / NANOS_PER_DAY;

        SimDate {
            year: year as i32,
            month: (month as u8) + 1,
            day: (day as u8) + 1,
        }
    }

    /// Formats the time as an RFC 3339 string with nanosecond precision.
    ///
    /// # Returns
    ///
    /// A `String` in RFC 3339 format (e.g., "2024-01-01T00:00:00.000000000Z")
    pub fn format_rfc3339(&self) -> String {
        self.to_datetime()
            .to_rfc3339_opts(SecondsFormat::Nanos, true)
    }

    // ----------------------------------------------------
    // Arithmetic
    // ----------------------------------------------------

    /// Adds nanoseconds to the current time.
    ///
    /// # Arguments
    ///
    /// * `ns` - The number of nanoseconds to add (can be negative)
    ///
    /// # Returns
    ///
    /// A new `SimTime` with the added nanoseconds
    pub fn add_ns(self, ns: i128) -> Self {
        SimTime(self.0 + ns)
    }

    /// Adds seconds to the current time.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of seconds to add (can be negative)
    ///
    /// # Returns
    ///
    /// A new `SimTime` with the added seconds
    pub fn add_seconds(self, n: i64) -> Self {
        SimTime(self.0 + seconds(n))
    }

    /// Adds minutes to the current time.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of minutes to add (can be negative)
    ///
    /// # Returns
    ///
    /// A new `SimTime` with the added minutes
    pub fn add_minutes(self, n: i64) -> Self {
        SimTime(self.0 + minutes(n))
    }

    /// Adds hours to the current time.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of hours to add (can be negative)
    ///
    /// # Returns
    ///
    /// A new `SimTime` with the added hours
    pub fn add_hours(self, n: i64) -> Self {
        SimTime(self.0 + hours(n))
    }

    /// Adds days to the current time.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of days to add (can be negative)
    ///
    /// # Returns
    ///
    /// A new `SimTime` with the added days
    pub fn add_days(self, n: i64) -> Self {
        SimTime(self.0 + days(n))
    }

    /// Adds weeks to the current time.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of weeks to add (can be negative)
    ///
    /// # Returns
    ///
    /// A new `SimTime` with the added weeks
    pub fn add_weeks(self, n: i64) -> Self {
        SimTime(self.0 + weeks(n))
    }

    /// Adds months to the current time.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of months to add (can be negative)
    ///
    /// # Returns
    ///
    /// A new `SimTime` with the added months
    pub fn add_months(self, n: i64) -> Self {
        SimTime(self.0 + months(n))
    }

    /// Adds years to the current time.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of years to add (can be negative)
    ///
    /// # Returns
    ///
    /// A new `SimTime` with the added years
    pub fn add_years(self, n: i64) -> Self {
        SimTime(self.0 + years(n))
    }

    /// Adds a `SimDuration` to the current time.
    ///
    /// # Arguments
    ///
    /// * `dt` - The duration to add
    ///
    /// # Returns
    ///
    /// A new `SimTime` with the added duration
    #[inline]
    pub fn add(self, dt: SimDuration) -> Self {
        SimTime(self.0 + dt.0)
    }
    
    /// Returns the number of ticks (nanoseconds) in the specified unit.
    ///
    /// # Arguments
    ///
    /// * `_unit` - The unit of time (currently unused)
    ///
    /// # Returns
    ///
    /// The number of nanoseconds as an `i128`
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




pub fn deserialize_simtime<'de, D>(d: D) -> Result<SimTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    let ns = s.parse::<i128>().map_err(D::Error::custom)?;
    Ok(SimTime(ns))
}

pub fn deserialize_simtime_opt<'de, D>(
    d: D,
) -> Result<Option<SimTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(d)?;
    match opt {
        Some(s) => {
            let ns = s.parse::<i128>().map_err(serde::de::Error::custom)?;
            Ok(Some(SimTime(ns)))
        }
        None => Ok(None),
    }
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

// ------------------------------------------------------------
// Default impl
// ------------------------------------------------------------
impl Default for SimTime {
    fn default() -> Self {
        SimTime(0)
    }
}
// ------------------------------------------------------------
// Arithmetic impls
// ------------------------------------------------------------

impl SimTime {
    pub fn zero() -> Self { SimTime(0) }
}


use std::ops::{Add, Sub};

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

// Absolute subtraction produces a duration
impl Sub for SimTime {
    type Output = SimDuration;
    fn sub(self, rhs: SimTime) -> Self::Output {
        SimDuration(self.0 - rhs.0)
    }
}

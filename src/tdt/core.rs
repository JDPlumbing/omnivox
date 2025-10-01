use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeDelta {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}


impl TimeDelta {
    pub fn from_now() -> Self {
        let epoch = DateTime::<Utc>::from_timestamp(0, 0).unwrap();
        let now = Utc::now();
        Self { start: epoch, end: now }
    }

    pub fn between(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { start, end }
    }

    pub fn until_now(start: DateTime<Utc>) -> Self {
        let now = Utc::now();
        Self { start, end: now }
    }

    /// Count ticks in a given unit ("days", "hours", "minutes", etc.)
    pub fn ticks(&self, unit: &str) -> i64 {
        let delta = self.end - self.start;

        match unit {
            "days" => delta.num_days(),
            "hours" => delta.num_hours(),
            "minutes" => delta.num_minutes(),
            "seconds" => delta.num_seconds(),
            "milliseconds" => delta.num_milliseconds(),
            "microseconds" => delta.num_microseconds().unwrap_or(i64::MAX),
            "nanoseconds" => delta.num_nanoseconds().unwrap_or(i64::MAX),
            _ => panic!("Unsupported unit: {}", unit),
        }
    }

    /// Pretty breakdown like "27 years, 5 months, 18 days"
    pub fn pretty(&self, max_units: usize) -> String {
        let delta = self.end - self.start;
        let days = delta.num_days();

        let years = days / 365;
        let months = (days % 365) / 30;
        let days_left = (days % 365) % 30;

        let hours = delta.num_hours() % 24;
        let minutes = delta.num_minutes() % 60;
        let seconds = delta.num_seconds() % 60;

        let mut parts = vec![];
        if years > 0 {
            parts.push(format!("{} year{}", years, if years != 1 { "s" } else { "" }));
        }
        if months > 0 {
            parts.push(format!("{} month{}", months, if months != 1 { "s" } else { "" }));
        }
        if days_left > 0 {
            parts.push(format!("{} day{}", days_left, if days_left != 1 { "s" } else { "" }));
        }
        if hours > 0 {
            parts.push(format!("{} hour{}", hours, if hours != 1 { "s" } else { "" }));
        }
        if minutes > 0 {
            parts.push(format!("{} minute{}", minutes, if minutes != 1 { "s" } else { "" }));
        }
        if seconds > 0 && parts.is_empty() {
            parts.push(format!("{} second{}", seconds, if seconds != 1 { "s" } else { "" }));
        }

        let mut display = parts.into_iter().take(max_units).collect::<Vec<_>>();
        if display.is_empty() {
            display.push("0 seconds".to_string());
        }
        display.join(", ")
    }

    pub fn from_ticks(ticks: i64, unit: &str) -> Self {
        let duration = match unit {
            "nanoseconds" => chrono::Duration::nanoseconds(ticks),
            "microseconds" => chrono::Duration::microseconds(ticks),
            "milliseconds" => chrono::Duration::milliseconds(ticks),
            "seconds" => chrono::Duration::seconds(ticks),
            "minutes" => chrono::Duration::minutes(ticks),
            "hours" => chrono::Duration::hours(ticks),
            _ => chrono::Duration::nanoseconds(ticks), // default fallback
        };

        let epoch = DateTime::<Utc>::from_timestamp(0, 0).unwrap();
        Self {
            start: epoch,
            end: epoch + duration,
        }
    }

}

# tdt (Time Delta Toolkit)

The **tdt** module provides a deterministic, integer-based time system for simulations. It includes absolute timestamps (`SimTime`), durations (`SimDuration`), calendar conversions (`SimDate`), human-friendly formatting, event aggregation, and Julian date utilities. All time values are represented as nanoseconds since the Unix epoch, enabling precise, reproducible arithmetic over arbitrary time spans.

## Purpose

- **Deterministic simulation time:** No floating-point drift, no leap seconds, no astronomical irregularities.
- **Fixed calendar:** A simplified 365.25-day year divided into 12 equal months, 7-day weeks, and 24-hour days.
- **Event bucketing:** Group simulation events by month, week, day, or hour for analysis and visualization.
- **Human-readable output:** Convert nanosecond timestamps into RFC3339, YYYY-MM-DD, or friendly duration strings.
- **Julian date support:** Convert between simulation time and Julian Day Numbers for astronomical calculations.

## Key Files

### **sim_time.rs**

Defines `SimTime`, the core absolute timestamp type.

**Features:**
- **Constructors:** `from_ns`, `from_seconds`, `from_datetime`, `from_sim_date`, `now`
- **Conversions:** `as_ns`, `to_datetime`, `to_sim_date`, `format_rfc3339`
- **Arithmetic:** `add_ns`, `add_seconds`, `add_minutes`, `add_hours`, `add_days`, `add_weeks`, `add_months`, `add_years`, `add(SimDuration)`
- **Operators:** `+`, `-` (with `SimDuration`), subtraction between two `SimTime` produces `SimDuration`

**Example:**
```rust
use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_duration::SimDuration;

let t0 = SimTime::now();
let t1 = t0.add_days(30);
let delta: SimDuration = t1 - t0;
println!("Delta: {}", delta.to_string_human());
```

### **sim_duration.rs**

Defines `SimDuration`, representing time intervals as `i128` nanoseconds.

**Features:**
- **Constructors:** `from_ns`, `from_seconds`, `seconds`, `minutes`, `hours`, `days`, `weeks`, `months`, `years`
- **Accessors:** `as_ns`, `seconds_f64`, `is_zero`, `ZERO`
- **Human formatting:** `to_string_human()` (e.g., "2.5 hours", "30 seconds")
- **Arithmetic:** `+`, `-`, `*`, `/` (with `i64` or `i128`)

**Example:**
```rust
use crate::core::tdt::sim_duration::SimDuration;

let dur = SimDuration::hours(3) + SimDuration::minutes(45);
println!("{}", dur.to_string_human()); // "3.75 h"
```

### **sim_date.rs**

Defines `SimDate`, a human-friendly YYYY-MM-DD date within the fixed simulation calendar.

**Features:**
- `new(year, month, day)` — Safe constructor with bounds checking.
- `to_sim_time()` — Convert to absolute `SimTime`.
- Display impl for formatting as YYYY-MM-DD.

**Example:**
```rust
use crate::core::tdt::sim_date::SimDate;

let date = SimDate::new(2025, 12, 11);
let t = date.to_sim_time();
println!("Date: {}, SimTime: {}", date, t.as_ns());
```

### **sim_calendar.rs**

Defines fixed nanosecond constants for all time units.

**Constants:**
- `NANOS_PER_SECOND`, `NANOS_PER_MINUTE`, `NANOS_PER_HOUR`, `NANOS_PER_DAY`
- `NANOS_PER_WEEK` (7 days)
- `NANOS_PER_MONTH` (1/12 of a year)
- `NANOS_PER_YEAR` (365.25 days)

**Helper functions:**
- `seconds(n)`, `minutes(n)`, `hours(n)`, `days(n)`, `weeks(n)`, `months(n)`, `years(n)` — Convert counts to nanoseconds.

**Example:**
```rust
use crate::core::tdt::sim_calendar::*;

let one_week_ns = weeks(1);
let one_month_ns = months(1);
```

### **sim_display.rs**

Human and machine-friendly formatting for `SimTime`.

**TimeFormat enum:**
- `RawNs` — Raw nanoseconds as `i128`.
- `Rfc3339` — Real datetime (mapped to Unix epoch).
- `SimDate` — YYYY-MM-DD (deterministic sim calendar).
- `SimMonth` — Zero-based month index.
- `SimWeek` — Zero-based week index.
- `SimDay` — Zero-based day index.

**Functions:**
- `format_simtime(t, fmt)` — Format a `SimTime` using the chosen format.
- `sim_month_index(t)`, `sim_week_index(t)`, `sim_day_index(t)`, `sim_hour_index(t)` — Zero-based indices.
- `format_simdate(t)` — YYYY-MM-DD string.
- `rfc3339_from_simtime(t)` — RFC3339 timestamp.
- `format_event_block(events, fmt)` — Group multiple `ChronoEvent`s under a single displayed timestamp.

**Example:**
```rust
use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_display::{format_simtime, TimeFormat};

let t = SimTime::now();
println!("SimDate: {}", format_simtime(t, TimeFormat::SimDate));
println!("RFC3339: {}", format_simtime(t, TimeFormat::Rfc3339));
```

### **sim_aggregate.rs**

Deterministic time-bucket aggregation for `ChronoEvent`s.

**AggregateResolution enum:**
- `RawNs` — No aggregation (full fidelity).
- `SimMonth` — Bucket by month index.
- `SimWeek` — Bucket by week index.
- `SimDay` — Bucket by day index.
- `SimHour` — Bucket by hour index.

**Functions:**
- `aggregate_events(events, resolution)` — Returns `Vec<serde_json::Value>` with bucketed events.
- `aggregate_events_json_pretty(events, resolution)` — Pretty JSON string.
- `aggregate_events_json(events, resolution)` — Compact JSON string.

**Example:**
```rust
use crate::core::chronovox::ChronoEvent;
use crate::core::tdt::sim_aggregate::{aggregate_events, AggregateResolution};

let events: Vec<ChronoEvent> = /* ... */;
let buckets = aggregate_events(&events, AggregateResolution::SimDay);
println!("{}", serde_json::to_string_pretty(&buckets).unwrap());
```

### **time_delta.rs**

Legacy helper for computing time intervals using `chrono::DateTime<Utc>`. Provides `TimeDelta` with human-friendly breakdown and conversion from `SimDuration`.

**Features:**
- `from_now()`, `between(start, end)`, `until_now(start)`, `from_ticks(ticks, unit)`, `from_sim_duration(dur)`
- `ticks(unit)` — Count ticks in days, hours, minutes, seconds, etc.
- `pretty(max_units)` — Human-readable breakdown (e.g., "2 years, 3 months, 5 days").

**Example:**
```rust
use crate::core::tdt::time_delta::TimeDelta;
use chrono::Utc;

let delta = TimeDelta::from_now();
println!("Elapsed: {}", delta.pretty(3));
```

## Usage Summary

- **Absolute time:** Use `SimTime` for timestamps, `SimDuration` for intervals.
- **Arithmetic:** Add/subtract durations, compute deltas between timestamps.
- **Formatting:** Use `format_simtime` for display, `to_string_human` for durations.
- **Event aggregation:** Use `aggregate_events` to group events by month, week, day, or hour.
- **Julian dates:** Use `simtime_to_julian` for astronomical conversions.

## Conventions

- All time values are in nanoseconds (`i128`) for precision and determinism.
- The fixed calendar is 365.25 days/year, 12 equal months, 7-day weeks.
- No leap seconds, no astronomical corrections—purely deterministic.
- Serialization uses `serde` for all core types.

## Extending

- Add new time units by extending `sim_calendar.rs`.
- Add new `TimeFormat` variants in `sim_display.rs` for custom output.
- Add new `AggregateResolution` modes for finer or coarser bucketing.

---

**Maintainer:** drippy  
**License:** MIT
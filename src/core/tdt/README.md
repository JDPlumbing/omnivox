# tdt Module

The `tdt` (Time Delta Toolkit) module provides utilities for representing, manipulating, and formatting time intervals in Rust. It is designed for use in simulations, event sourcing, and any application where precise or human-friendly time deltas are needed.

## Features

- **TimeDelta Struct:**  
  Represents a time interval between two `DateTime<Utc>` points.
- **Convenient Constructors:**  
  - `from_now()`: Interval from the Unix epoch to now.
  - `between(start, end)`: Interval between two times.
  - `until_now(start)`: Interval from a given time to now.
  - `from_ticks(ticks, unit)`: Interval from a number of ticks in a given unit.
- **Tick Counting:**  
  - `ticks(unit)`: Returns the interval length in days, hours, minutes, seconds, milliseconds, microseconds, or nanoseconds.
- **Pretty Formatting:**  
  - `pretty(max_units)`: Returns a human-readable breakdown (e.g., "2 years, 3 months, 5 days").
- **Serialization:**  
  - Implements `Serialize` and `Deserialize` for easy storage and transfer.

## Example Usage

```rust
use tdt::core::TimeDelta;
use chrono::Utc;

// Interval from epoch to now
let delta = TimeDelta::from_now();
println!("Elapsed days: {}", delta.ticks("days"));
println!("Pretty: {}", delta.pretty(3));

// Interval between two times
let start = Utc::now();
let end = start + chrono::Duration::days(10);
let delta = TimeDelta::between(start, end);
println!("Interval: {} days", delta.ticks("days"));

// From ticks
let delta = TimeDelta::from_ticks(3600, "seconds");
println!("Pretty: {}", delta.pretty(2));
# TDT (Time Delta Toolkit)

[![Crates.io](https://img.shields.io/crates/v/tdt.svg)](https://crates.io/crates/tdt)
[![Documentation](https://docs.rs/tdt/badge.svg)](https://docs.rs/tdt)
[![License](https://img.shields.io/crates/l/tdt.svg)](./LICENSE)

**Time Delta Toolkit (TDT)** — a tiny Rust library for measuring elapsed time between events.  
Designed for simulations, games, and any system where you need fast, flexible time deltas.

---

## ✨ Features

- **Simple API** with the `TimeDelta` struct
- **Nanosecond-level precision** (via `chrono`)
- Multiple constructors:
  - `from_now()` — since epoch until now
  - `between(start, end)` — between two datetimes
  - `until_now(start)` — from a given datetime until now
- Utilities:
  - `.ticks(unit)` → elapsed in a unit (`days`, `hours`, `minutes`, `seconds`, `milliseconds`, `microseconds`, `nanoseconds`)
  - `.pretty(max_units)` → human-readable breakdown like `"25 years, 3 months, 12 days"`

---

## 📦 Installation

```bash
cargo add tdt
```

Or add manually to `Cargo.toml`:

```toml
[dependencies]
tdt = "0.1"
```

---

## 🔍 Example

```rust
use chrono::Utc;
use tdt::core::TimeDelta;

fn main() {
    let start = Utc::now();
    let td = TimeDelta::until_now(start);

    println!("Elapsed (seconds): {}", td.ticks("seconds"));
    println!("Pretty: {}", td.pretty(3));
}
```

Output:
```
Elapsed (seconds): 42
Pretty: 0 minutes, 42 seconds
```

---

## 📖 API

### `TimeDelta::from_now() -> TimeDelta`
Construct a delta from the epoch (`1970-01-01 00:00:00 UTC`) until now.

### `TimeDelta::between(start: DateTime<Utc>, end: DateTime<Utc>) -> TimeDelta`
Construct a delta between two arbitrary datetimes.

### `TimeDelta::until_now(start: DateTime<Utc>) -> TimeDelta`
Construct a delta from a given datetime until now.

### `TimeDelta::ticks(unit: &str) -> i64`
Return elapsed time in the specified unit. Supported:
`"days" | "hours" | "minutes" | "seconds" | "milliseconds" | "microseconds" | "nanoseconds"`

### `TimeDelta::pretty(max_units: usize) -> String`
Return a human-readable string breakdown, up to `max_units` parts.

---

## 🛠 Development

Run tests:

```bash
cargo test
```

Run benchmarks:

```bash
cargo bench
```

Format code:

```bash
cargo fmt
```

---

## 📎 Links
- [Crates.io](https://crates.io/crates/tdt)
- [Docs.rs](https://docs.rs/tdt)
- [Source Code](https://github.com/JDPlumbing/tdt-rs)

---

## 📄 License

MIT © JD Plumbing

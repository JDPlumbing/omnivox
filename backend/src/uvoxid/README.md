# uvoxid

[![Crates.io](https://img.shields.io/crates/v/uvoxid.svg)](https://crates.io/crates/uvoxid)
[![Documentation](https://docs.rs/uvoxid/badge.svg)](https://docs.rs/uvoxid)
[![License](https://img.shields.io/crates/l/uvoxid.svg)](./LICENSE)
A crate for representing positions in space (and time) with **pure integers**, designed for simulations, physics, and spatial indexing.

## Overview

`UvoxId` encodes a position as **four 64-bit fields**:

- `frame_id: u64` → Reference frame (0 = Earth, 1 = Moon, 2 = Sun, …)
- `r_um: u64` → Radial distance from the frame center, in micrometers
- `lat_code: i64` → Latitude code (full 64-bit signed integer range)
- `lon_code: i64` → Longitude code (full 64-bit signed integer range)

This gives:
- **Integer math only** (no floats in the core representation).
- **Ridiculous precision** — sub-atomic resolution at Earth’s surface, ~6 mm resolution even at 2 light-years.
- **Stable hex serialization** (256-bit fixed-width string).
- Optional `serde` support for JSON/CBOR serialization.

## Examples

```rust
use uvoxid::{UvoxId, Delta};

// Construct a position (frame 0 = Earth)
let mut pos = UvoxId::earth(6_371_000_000_000, 0, 0);

// Apply a delta
let delta = Delta { dr_um: 100, dlat: 50, dlon: -50 };
pos.apply_delta(delta);

println!("{}", pos); 
// → frame=0, r=6371000000100 µm, lat=50, lon=-50

// Serialize to hex
let hex = pos.to_hex();
let back = UvoxId::from_hex(&hex).unwrap();
assert_eq!(pos, back);

// Serialize to JSON (with serde enabled)
let json = serde_json::to_string(&pos).unwrap();
let decoded: UvoxId = serde_json::from_str(&json).unwrap();
assert_eq!(pos, decoded);
```

## Why?

- **Consistent, integer-only units** for distance, area, volume, velocity, and time.
- **Reference-frame aware**: anchor at Earth, Moon, Sun, or any body by `frame_id`.
- **Simulation-ready**: can represent velocity as “uvox per tick,” where one tick = 1 ns, with the speed of light defined as a max velocity in those units.

## Benchmarks

With Criterion:

- Construct `UvoxId`: ~2 ns  
- Apply delta: ~8 ns  
- Serialize/deserialize JSON: ~50–100 ns  
- Haversine distance: ~37 ns  


---

## 📎 Links
- [Crates.io](https://crates.io/crates/uvoxid)
- [Docs.rs](https://docs.rs/uvoxid)
- [Source Code](https://github.com/JDPlumbing/uvoxid-rs)

---

## 📄 License

MIT © JD Plumbing

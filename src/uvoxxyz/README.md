# uvoxxyz

[![CI](https://github.com/JDPlumbing/uvoxxyz/actions/workflows/ci.yml/badge.svg)](https://github.com/JDPlumbing/uvoxxyz/actions)

Created by [JDPlumbing](https://github.com/JDPlumbing)

## 📦 Installation

```bash
cargo install uvoxxyz
```

**Bridge between spherical `UvoxId` coordinates and Cartesian space, with quaternion support.**

`uvoxxyz` lets you:
- Convert between [`uvoxid`](https://crates.io/crates/uvoxid) spherical IDs and 3D Cartesian coordinates.
- Choose between **math convention** (X/Y horizontal, Z up) and **graphics convention** (X/Z horizontal, Y up).
- Apply quaternions for orientation and rotation in either system.


---

## Example

```rust
use uvoxid::UvoxId;
use uvoxxyz::types::{CoordSystem, Cartesian, Quaternion};
use uvoxxyz::convert::*;

fn main() {
    // A UvoxId located at radius=1m, lat=0, lon=0
    let id = UvoxId::new(0, 1_000_000, 0, 0);

    // Convert to Cartesian (math convention)
    let cart = id.to_cartesian(CoordSystem::Math);
    println!("Cartesian: ({}, {}, {})", cart.x, cart.y, cart.z);

    // Rotate by 90° around Z
    let q = Quaternion::from_axis_angle(
        Cartesian { x: 0.0, y: 0.0, z: 1.0 },
        std::f64::consts::FRAC_PI_2
    );
    let rotated = q.rotate(cart);
    println!("Rotated: ({}, {}, {})", rotated.x, rotated.y, rotated.z);

    // Convert back to UvoxId
    let round_trip = UvoxId::from_cartesian(rotated, CoordSystem::Math, 0);
    println!("Back to UvoxId: {}", round_trip);
}
```

---

## Features
- 🧭 `UvoxId → Cartesian` and back
- 🔄 Support for **math** and **graphics** axis conventions
- 🎛️ Quaternion math (normalize, multiply, rotate)
- ⚡ Nanosecond-level benchmarks (sub-20ns conversions)

---

## Related Crates
- [`uvoxid`](https://crates.io/crates/uvoxid): Hierarchical spherical integer coordinate system
- [`geospec`](https://crates.io/crates/geospec): Geometric shapes and inference utilities

---

## License
MIT or Apache-2.0, at your option.

---

## Minimum Rust Version
Rust 1.70+

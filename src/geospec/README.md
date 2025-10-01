# geospec

`geospec` is a geometry specification library designed for ECS-style simulations.  
It provides **shape definitions**, **dimension inference**, and **JSON-friendly representations** for physical objects.

---

## Features
- Define common geometric shapes:
  - Sphere (radius, volume, surface area)
  - Box (length × width × height, volume, surface area)
  - Cylinder (radius × height, volume, surface area)
  - Cone (radius × height, volume, surface area)
  - Pyramid (base × height, volume, surface area)
  - Capsule, Torus, Prism, etc. (planned)

- Infer missing properties:
  - Provide just a radius → auto-compute volume and surface area.
  - Provide dimensions → auto-compute derived measures.

- Serialize/deserialize to **JSON** using `serde_json`.

- Optimized for **speed**: most computations run in nanoseconds.

---

## Example

```rust
use geospec::shapes::Sphere;
use geospec::traits::ShapeSpec;

fn main() {
    let sphere = Sphere { radius: 2.0 };
    let json = sphere.as_json();

    println!("Sphere as JSON: {}", json);
    println!("Volume: {}", sphere.volume());
    println!("Surface area: {}", sphere.surface_area());
}
```

Output:
```
Sphere as JSON: {"radius":2.0,"volume":33.510,"surface_area":50.265}
Volume: 33.510
Surface area: 50.265
```

---

## Benchmarks

Run with:

```bash
cargo bench
```

Typical performance (on a mid-range CPU):
- Sphere volume: ~1.4 ns
- Sphere surface area: ~1.1 ns
- Box volume: ~0.46 ns
- Box surface area: ~0.44 ns

---

## License
Dual-licensed under either:
- MIT License
- Apache License 2.0

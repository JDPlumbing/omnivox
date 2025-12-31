# geospec Module

The `geospec` ( GEOmetric SPECification) module provides geometric primitives, traits, and inference utilities for representing and working with basic shapes in simulations and scientific applications. It enables calculation of surface area, volume, and serialization of geometric data, as well as inferring missing properties from partial JSON input.

## Features

- **Geometric Primitives:** Includes common shapes such as `Point`, `Line`, `Plane`, `Sphere`, `BoxShape`, `Cylinder`, and `Cone`.
- **Traits:**  
  - `SurfaceArea`: For types that can compute their surface area.
  - `Volume`: For types that can compute their volume.
  - `Dimensions`: For types that can serialize their dimensions and properties as JSON.
- **Inference Utility:**  
  - `infer_from_json`: Given partial JSON describing a shape, infers and fills in missing properties (like surface area and volume).

## Structure

- **traits.rs:** Defines the core traits (`SurfaceArea`, `Volume`, `Dimensions`) for geometric types.
- **shapes.rs:** Implements the geometric primitives and their trait methods.
- **inference.rs:** Provides the `infer_from_json` function for inferring shape properties from JSON.

## Example Usage

```rust
use geospec::{Sphere, Dimensions};
use serde_json::json;

// Create a sphere and get its properties as JSON
let sphere = Sphere { radius: 2.0 };
let props = sphere.as_json();
println!("{}", props);

// Infer properties from partial JSON
use geospec::inference::infer_from_json;
let input = json!({ "type": "sphere", "radius": 2.0 });
let inferred = infer_from_json(&input).unwrap();
println!("{}", inferred);
# uvoxxyz Module

The `uvoxxyz` module provides utilities for 3D spatial math, coordinate system conversion, and orientation for use with the `UvoxId` spatial identifier. It enables seamless conversion between compact integer-based spatial IDs and floating-point Cartesian coordinates, supports multiple coordinate system conventions, and provides tools for local ENU (East-North-Up) frames and quaternion-based orientation.

## Features

- **Coordinate Conversion:**  
  Convert between `UvoxId` (integer-based spatial ID) and `Cartesian` (x, y, z) coordinates in both "Math" (Z-up) and "Graphics" (Y-up) conventions.
- **Local ENU Frames:**  
  Compute local East-North-Up (ENU) bases at any anchor point, and transform points between global and local ENU coordinates.
- **Quaternion Orientation:**  
  Minimal `Quat` struct for 3D orientation, with axis-angle construction, normalization, and vector rotation.
- **CoordSystem Enum:**  
  Explicitly distinguishes between mathematical (Z-up) and graphics (Y-up) coordinate systems for compatibility with physics engines and rendering pipelines.

## Structure

- **types.rs:**  
  Defines the `Cartesian` struct for 3D points and the `CoordSystem` enum.
- **convert.rs:**  
  Extension trait for converting between `UvoxId` and `Cartesian` coordinates in either coordinate system.
- **enu.rs:**  
  Functions for building ENU bases, and for converting between global and local ENU coordinates.
- **quat.rs:**  
  Minimal quaternion implementation for representing and applying 3D rotations.
- **mod.rs:**  
  Module exports for easy integration.

## Example Usage

```rust
use uvoxid::UvoxId;
use uvoxxyz::{Cartesian, CoordSystem, UvoxIdExt, Quat, enu_basis, to_local_enu, from_local_enu};

// Convert UvoxId to Cartesian coordinates (Z-up)
let id = UvoxId::earth(1_000_000, 123_456, -654_321);
let cart = id.to_cartesian(CoordSystem::Math);

// Convert back to UvoxId
let id2 = UvoxId::from_cartesian(cart, CoordSystem::Math, id.frame_id);

// Build ENU basis at anchor
let (east, north, up) = enu_basis(&id);

// Transform a point to local ENU coordinates
let local = to_local_enu(&id, &id2);

// Quaternion rotation
let q = Quat::from_axis_angle(Cartesian { x: 0.0, y: 0.0, z: 1.0 }, std::f64::consts::PI / 2.0);
let rotated = q.rotate(cart);
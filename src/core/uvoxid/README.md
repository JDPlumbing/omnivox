# uvoxid Module

The `uvoxid` module provides a compact, lossless, and arithmetic-friendly identifier for spatial locations within arbitrary reference frames. It is designed for use in simulations, games, or scientific applications where precise, serializable, and math-capable spatial IDs are needed.

## Features

- **UvoxId Struct:**  
  Represents a spatial location as four 64-bit fields:
  - `frame_id`: Reference frame anchor (e.g., Earth, Moon, Sun, etc.)
  - `r_um`: Radial distance from frame center, in micrometers
  - `lat_code`: Latitude code (full 64-bit signed range)
  - `lon_code`: Longitude code (full 64-bit signed range)
- **Delta Struct:**  
  Represents a difference between two `UvoxId`s, supporting arithmetic operations and safe wrapping for latitude/longitude.
- **Arithmetic Operations:**  
  - Add or apply a `Delta` to a `UvoxId` (with safe wrapping and clamping).
- **Serialization:**  
  - Convert to/from a packed 256-bit hex string for efficient storage or transmission.
- **Convenience Methods:**  
  - Construct for Earth or other frames.
  - Tuple conversion for math or serialization.
  - Safe latitude/longitude wrapping and clamping.

## Structure

- **core.rs:** Defines the `UvoxId` struct, arithmetic, and serialization logic.
- **delta.rs:** Defines the `Delta` struct and its helpers.
- **mod.rs:** Module exports for easy integration.

## Example Usage

```rust
use uvoxid::{UvoxId, Delta};

// Create a UvoxId for Earth
let id = UvoxId::earth(1_000_000, 123_456, -654_321);

// Apply a delta (move 1000 µm radially, +100 lat, -200 lon)
let delta = Delta::new(1000, 100, -200);
let new_id = id + delta;

// Serialize to hex and back
let hex = new_id.to_hex();
let parsed = UvoxId::from_hex(&hex).unwrap();

assert_eq!(parsed, new_id);
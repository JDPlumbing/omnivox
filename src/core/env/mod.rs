//! Omnivox Global Environment Module
//!
//! Provides:
//!   - Planetary bodies (Earth, Sun, Moon)
//!   - Gravity fields (inverse-square)
//!   - Atmospheric model (simple density + layer classification)
//!   - Solar flux + radiation
//!   - Environment safety classification (vacuum, mantle, core)
//!
//! These functions are stateless and extremely fast.
//! They DO NOT touch ECS. Systems call them.

pub mod bodies;
pub mod gravity;
pub mod safety;
pub mod solar;
pub mod atmosphere;

pub use bodies::*;
pub use gravity::*;
pub use safety::*;
pub use solar::*;
pub use atmosphere::*;

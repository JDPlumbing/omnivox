//! Rendering is a projection layer.
//!
//! It consumes simulation state and produces visual representations.
//! It does not own time, physics, or world meaning.


pub mod view;
pub mod primitives;
pub mod adapters;
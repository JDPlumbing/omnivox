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



pub mod atmosphere;
pub mod fields;
pub mod gravity;
pub mod medium;
pub mod pressure;
pub mod resistance;
pub mod temperature;
pub mod env_snapshot;
pub mod derived_env;



pub use gravity::*;
pub use atmosphere::*;
pub use medium::*;
pub use pressure::*;
pub use resistance::*;
pub use temperature::*;
pub use fields::{Field, FieldSample};
pub use env_snapshot::{EnvSnapshot, sample_environment};
pub use derived_env::DerivedEnv;
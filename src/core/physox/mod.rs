//! # Physox — The Laws of Motion and Energy
//!
//! This module defines the physical laws that govern how objects move,
//! interact, and exchange energy inside the Omnivox simulation.
//!
//! It does **not** store simulation state. Instead, it provides reusable,
//! deterministic rules for kinematics, dynamics, energy, and interactions.

pub mod constants;
pub mod kinematics;
pub mod dynamics;
pub mod energy;
pub mod interaction;
pub mod astronomy;

// Convenient re-exports
pub use constants::*;
pub use kinematics::*;
pub use dynamics::*;
pub use energy::*;
pub use interaction::*;
pub use astronomy::*;
// core/cosmic/components/orbit.rs
use crate::core::cosmic::id::CosmicBodyId;
use crate::core::physics::units::{length::Meters, time::Seconds, angle::Radians};

#[derive(Debug, Clone, Copy)]
pub struct Orbit {
    /// Body this one orbits (Sun, planet, etc.)
    pub primary: CosmicBodyId,

    /// Semi-major axis
    pub semi_major_axis: Meters,

    /// Orbital period
    pub period: Seconds,

    /// Inclination relative to parent frame
    pub inclination: Radians,

    /// Phase at epoch
    pub phase_at_epoch: Radians,
}

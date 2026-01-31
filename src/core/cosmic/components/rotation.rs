// core/cosmic/components/rotation.rs
use crate::core::physics::units::{time::Seconds, angle::Radians};

#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    /// Sidereal rotation period
    pub period: Seconds,

    /// Rotation phase at epoch
    pub phase_at_epoch: Radians,
}

// core/cosmic/components/axial_tilt.rs
use crate::core::physics::units::angle::Radians;

#[derive(Debug, Clone, Copy)]
pub struct AxialTilt {
    pub radians: Radians,
}

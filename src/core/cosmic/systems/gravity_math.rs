use crate::core::math::vec3::Vec3;
use crate::core::physics::units::acceleration::MetersPerSecondSquared;

#[derive(Debug, Clone, Copy)]
pub struct GravitationalAcceleration {
    /// Direction of acceleration (unit vector)
    pub direction: Vec3,

    /// Magnitude of acceleration
    pub magnitude: MetersPerSecondSquared,
}

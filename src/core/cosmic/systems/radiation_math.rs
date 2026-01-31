// core/cosmic/systems/radiation_math.rs
use crate::core::math::vec3::Vec3;
use crate::core::physics::units::length::Meters;
use crate::core::physics::units::irradiance::WattsPerSquareMeter;

#[derive(Debug, Clone, Copy)]
pub struct RadiationSample {
    pub direction: Vec3, // unit vector: target → source
    pub distance: Meters,
    pub flux: WattsPerSquareMeter,
}

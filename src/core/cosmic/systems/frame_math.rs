//core/cosmic/systems/frame_math.rs
use crate::core::math::vec3::Vec3;
use crate::core::math::mat3::Mat3;

#[derive(Debug, Clone, Copy)]
pub struct CosmicPose {
    /// Position of the body origin in the inertial (cosmic) frame
    pub position: Vec3,

    /// Orientation mapping body-local → inertial frame
    pub orientation: Mat3,
}

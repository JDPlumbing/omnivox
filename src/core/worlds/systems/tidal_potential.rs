use crate::core::math::vec3::Vec3;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TidalPotential {
    /// Differential acceleration vector in world frame
    pub differential_acceleration: Vec3,
}

pub fn compute_tidal_potential(
    gravity_at_center: Vec3,
    gravity_at_surface: Vec3,
) -> Vec3 {
    gravity_at_surface - gravity_at_center
}

use serde::{Serialize, Deserialize};
use crate::core::math::vec3::Vec3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldOrientation {
    /// Unit vector pointing toward the world's north pole,
    /// expressed in the cosmic body's local frame
    pub north_pole: Vec3,
}

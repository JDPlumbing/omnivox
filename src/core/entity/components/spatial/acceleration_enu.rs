use serde::{Deserialize, Serialize};
use crate::core::physics::units::acceleration::MetersPerSecondSquared;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AccelerationENU {
    pub east: MetersPerSecondSquared,
    pub north: MetersPerSecondSquared,
    pub up: MetersPerSecondSquared,
}

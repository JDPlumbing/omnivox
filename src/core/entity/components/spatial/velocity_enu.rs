use serde::{Deserialize, Serialize};

use crate::core::physics::units::velocity::MetersPerSecond;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VelocityENU {
    pub east: MetersPerSecond,
    pub north: MetersPerSecond,
    pub up: MetersPerSecond,
}

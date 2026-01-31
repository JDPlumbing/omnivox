use serde::{Deserialize, Serialize};
use crate::core::physics::units::length::Meters;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PositionENU {
    pub east: Meters,
    pub north: Meters,
    pub up: Meters,
}

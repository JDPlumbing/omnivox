use serde::{Serialize, Deserialize};
use crate::core::physics::units::angle::Radians;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Location {
    pub latitude: Radians,
    pub longitude: Radians,
}

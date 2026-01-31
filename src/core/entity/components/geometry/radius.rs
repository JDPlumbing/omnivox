use crate::core::physics::units::length::Meters;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Radius(pub Meters);      // meters

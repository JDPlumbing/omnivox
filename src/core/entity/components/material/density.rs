use crate::core::physics::units::density::KilogramsPerCubicMeter;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Density(pub KilogramsPerCubicMeter);
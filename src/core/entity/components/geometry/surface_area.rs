use crate::core::physics::units::area::SquareMeters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SurfaceArea(pub SquareMeters);

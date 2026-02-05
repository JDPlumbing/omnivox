// core/physics/units/angle.rs
use serde::{Serialize, Deserialize};

/// Mathematical angular unit.
/// 
/// Used ONLY in physics and math layers.
/// Never exposed to world or UI code.

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Radians(pub f64);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Degrees(pub f64);

impl Radians {
    pub fn from_degrees(deg: Degrees) -> Self {
        Radians(deg.0.to_radians())
    }

    pub fn to_degrees(self) -> Degrees {
        Degrees(self.0.to_degrees())
    }
}

impl Degrees {
    pub fn from_radians(rad: Radians) -> Self {
        Degrees(rad.0.to_degrees())
    }

    pub fn to_radians(self) -> Radians {
        Radians(self.0.to_radians())
    }
}

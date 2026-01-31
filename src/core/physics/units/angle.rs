// core/physics/units/angle.rs
use serde::{Serialize, Deserialize};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Radians(pub f64);

// core/physics/units/angles.rs (or similar)
impl From<f64> for Radians {
    fn from(v: f64) -> Self {
        Radians(v)
    }
}

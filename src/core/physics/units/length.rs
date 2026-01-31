// core/physics/units/length.rs
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Meters(pub f64);

use std::ops::AddAssign;

impl AddAssign for Meters {
    fn add_assign(&mut self, rhs: Meters) {
        self.0 += rhs.0;
    }
}

use serde::{Deserialize, Serialize};
use std::ops::{AddAssign, Mul};

use crate::core::physics::units::time::Seconds;
use crate::core::physics::units::length::Meters;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MetersPerSecond(pub f64);

impl AddAssign for MetersPerSecond {
    fn add_assign(&mut self, rhs: MetersPerSecond) {
        self.0 += rhs.0;
    }
}

impl Mul<Seconds> for MetersPerSecond {
    type Output = Meters;

    fn mul(self, dt: Seconds) -> Meters {
        Meters(self.0 * dt.0)
    }
}

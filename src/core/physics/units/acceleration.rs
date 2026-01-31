// core/physics/units/acceleration.rs

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MetersPerSecondSquared(pub f64);


use std::ops::Mul;
use crate::core::physics::units::time::Seconds;
use crate::core::physics::units::velocity::MetersPerSecond;

impl Mul<Seconds> for MetersPerSecondSquared {
    type Output = MetersPerSecond;

    fn mul(self, dt: Seconds) -> MetersPerSecond {
        MetersPerSecond(self.0 * dt.0)
    }
}

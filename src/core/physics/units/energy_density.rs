use std::ops::Mul;
use std::ops::AddAssign;
use serde::{Serialize, Deserialize};
use crate::core::physics::units::time::Seconds;
use crate::core::physics::units::irradiance::WattsPerSquareMeter;


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct JoulesPerSquareMeter(pub f64);

impl Mul<Seconds> for WattsPerSquareMeter {
    type Output = JoulesPerSquareMeter;

    fn mul(self, dt: Seconds) -> JoulesPerSquareMeter {
        JoulesPerSquareMeter(self.0 * dt.0)
    }
}

impl AddAssign for JoulesPerSquareMeter {
    fn add_assign(&mut self, rhs: JoulesPerSquareMeter) {
        self.0 += rhs.0;
    }
}

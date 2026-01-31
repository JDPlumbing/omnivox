// core/physics/units/energy.rs
use std::ops::{Add, AddAssign};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct Joules(pub f64);


impl Add for Joules {
    type Output = Joules;

    fn add(self, rhs: Joules) -> Joules {
        Joules(self.0 + rhs.0)
    }
}

impl AddAssign for Joules {
    fn add_assign(&mut self, rhs: Joules) {
        self.0 += rhs.0;
    }
}

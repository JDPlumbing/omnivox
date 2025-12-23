// src/sim/components/velocity.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Velocity {
    pub dr: f64,
    pub dlat: f64,
    pub dlon: f64,
}

impl Velocity {
    pub fn new(dr: f64, dlat: f64, dlon: f64) -> Self {
        Self { dr, dlat, dlon }
    }
}

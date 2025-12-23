
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Acceleration {
    pub ar: f64,    // radial acceleration (up/down)
    pub alat: f64,  // north/south acceleration
    pub alon: f64,  // east/west acceleration
}

impl Acceleration {
    pub fn new(ar: f64, alat: f64, alon: f64) -> Self {
        Self { ar, alat, alon }
    }
}


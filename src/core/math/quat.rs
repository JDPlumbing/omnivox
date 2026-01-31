use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Quat {
    pub w: f64,
    pub x: f64,    
    pub y: f64,   
    pub z: f64,   
}

impl Quat {
    pub fn identity() -> Self {
        Self { w: 1.0, x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Default for Quat {
    fn default() -> Self {
        Self::identity()
    }
}
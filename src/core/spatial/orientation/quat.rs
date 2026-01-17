use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UvoxQuat {
    pub w: f64,
    pub x: f64,     // rotation around radial axis (ê_r)
    pub y: f64,   // rotation around north-south axis (ê_lat)
    pub z: f64,   // rotation around east-west axis (ê_lon)
}

impl UvoxQuat {
    pub fn identity() -> Self {
        Self { w: 1.0, x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Default for UvoxQuat {
    fn default() -> Self {
        Self::identity()
    }
}
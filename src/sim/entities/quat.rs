use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UvoxQuat {
    pub w: f64,
    pub r: f64,     // rotation around radial axis (ê_r)
    pub lat: f64,   // rotation around north-south axis (ê_lat)
    pub lon: f64,   // rotation around east-west axis (ê_lon)
}

impl UvoxQuat {
    pub fn identity() -> Self {
        Self { w: 1.0, r: 0.0, lat: 0.0, lon: 0.0 }
    }
}

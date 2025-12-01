// sim/components/camera.rs
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Camera {
    pub fov: f32,      // degrees
    pub near: f32,     // meters (local)
    pub far: f32,      // meters (local)
}

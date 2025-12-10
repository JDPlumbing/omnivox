// backend/components/quaternion.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QuaternionLocal {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl QuaternionLocal {
    pub fn identity() -> Self {
        Self { w: 1.0, x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn to_glam(&self) -> glam::Quat {
        glam::Quat::from_xyzw(self.x, self.y, self.z, self.w)
    }

    pub fn from_glam(q: glam::Quat) -> Self {
        Self { w: q.w, x: q.x, y: q.y, z: q.z }
    }
}

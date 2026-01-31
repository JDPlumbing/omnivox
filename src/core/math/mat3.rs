use std::ops::{Mul};
use crate::core::math::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    pub m: [Vec3; 3], // rows
}
impl Mul for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Mat3 {
        let r0 = Vec3::new(
            self.m[0].dot(Vec3::new(rhs.m[0].x, rhs.m[1].x, rhs.m[2].x)),
            self.m[0].dot(Vec3::new(rhs.m[0].y, rhs.m[1].y, rhs.m[2].y)),
            self.m[0].dot(Vec3::new(rhs.m[0].z, rhs.m[1].z, rhs.m[2].z)),
        );

        let r1 = Vec3::new(
            self.m[1].dot(Vec3::new(rhs.m[0].x, rhs.m[1].x, rhs.m[2].x)),
            self.m[1].dot(Vec3::new(rhs.m[0].y, rhs.m[1].y, rhs.m[2].y)),
            self.m[1].dot(Vec3::new(rhs.m[0].z, rhs.m[1].z, rhs.m[2].z)),
        );

        let r2 = Vec3::new(
            self.m[2].dot(Vec3::new(rhs.m[0].x, rhs.m[1].x, rhs.m[2].x)),
            self.m[2].dot(Vec3::new(rhs.m[0].y, rhs.m[1].y, rhs.m[2].y)),
            self.m[2].dot(Vec3::new(rhs.m[0].z, rhs.m[1].z, rhs.m[2].z)),
        );

        Mat3 { m: [r0, r1, r2] }
    }
}
impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.m[0].x * v.x + self.m[0].y * v.y + self.m[0].z * v.z,
            self.m[1].x * v.x + self.m[1].y * v.y + self.m[1].z * v.z,
            self.m[2].x * v.x + self.m[2].y * v.y + self.m[2].z * v.z,
        )
    }
}


impl Mat3 {
    pub fn identity() -> Self {
        Self {
            m: [
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ],
        }
    }

    pub fn rotation_x(theta: f64) -> Self {
        let (s, c) = theta.sin_cos();
        Self {
            m: [
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0,  c, -s),
                Vec3::new(0.0,  s,  c),
            ],
        }
    }

    pub fn rotation_y(theta: f64) -> Self {
        let (s, c) = theta.sin_cos();
        Self {
            m: [
                Vec3::new( c, 0.0, s),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(-s, 0.0, c),
            ],
        }
    }

    pub fn rotation_z(theta: f64) -> Self {
        let (s, c) = theta.sin_cos();
        Self {
            m: [
                Vec3::new( c, -s, 0.0),
                Vec3::new( s,  c, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ],
        }
    }
}
pub fn rotate_around_axis(v: Vec3, axis: Vec3, angle: f64) -> Vec3 {
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    let axis = axis.normalized();

    let cross = axis.cross(v);
    let dot = axis.dot(v);

    v * cos_a + cross * sin_a + axis * dot * (1.0 - cos_a)
}
impl Mat3 {
    pub fn transpose(self) -> Mat3 {
        Mat3 {
            m: [
                Vec3::new(
                    self.m[0].x,
                    self.m[1].x,
                    self.m[2].x,
                ),
                Vec3::new(
                    self.m[0].y,
                    self.m[1].y,
                    self.m[2].y,
                ),
                Vec3::new(
                    self.m[0].z,
                    self.m[1].z,
                    self.m[2].z,
                ),
            ],
        }
    }

    /// For rotation matrices, inverse == transpose
    pub fn inverse(self) -> Mat3 {
        self.transpose()
    }
}

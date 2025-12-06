use crate::uvoxxyz::types::Cartesian;

/// Minimal quaternion struct for 3D orientation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Quat {
    /// Identity quaternion (no rotation).
    pub fn identity() -> Self {
        Self { w: 1.0, x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Build quaternion from axis + angle (in radians).
    pub fn from_axis_angle(axis: Cartesian, angle_rad: f64) -> Self {
        let half = angle_rad * 0.5;
        let (s, c) = half.sin_cos();
        let norm = (axis.x.powi(2) + axis.y.powi(2) + axis.z.powi(2)).sqrt();

        if norm == 0.0 {
            return Self::identity();
        }

        let ax = axis.x / norm;
        let ay = axis.y / norm;
        let az = axis.z / norm;

        Self { w: c, x: ax * s, y: ay * s, z: az * s }
    }

    /// Normalize the quaternion (important for avoiding drift).
    pub fn normalize(&self) -> Self {
        let mag = (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if mag == 0.0 {
            return Self::identity();
        }
        Self {
            w: self.w / mag,
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    /// Rotate a Cartesian vector by this quaternion.
    pub fn rotate(&self, v: Cartesian) -> Cartesian {
        let q = self.normalize();
        let u = Cartesian { x: q.x, y: q.y, z: q.z };
        let s = q.w;

        let dot_uv = u.x * v.x + u.y * v.y + u.z * v.z;
        let cross_uv = Cartesian {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        };

        Cartesian {
            x: 2.0 * dot_uv * u.x + (s * s - u.x * u.x - u.y * u.y - u.z * u.z) * v.x + 2.0 * (s * cross_uv.x + u.y * v.y + u.z * v.z),
            y: 2.0 * dot_uv * u.y + (s * s - u.x * u.x - u.y * u.y - u.z * u.z) * v.y + 2.0 * (s * cross_uv.y + u.z * v.z + u.x * v.x),
            z: 2.0 * dot_uv * u.z + (s * s - u.x * u.x - u.y * u.y - u.z * u.z) * v.z + 2.0 * (s * cross_uv.z + u.x * v.x + u.y * v.y),
        }
    }
}

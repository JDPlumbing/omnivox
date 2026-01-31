use crate::core::math::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct LocalTangentFrame {
    pub t1: Vec3,
    pub t2: Vec3,
    pub normal: Vec3,
}

/// Build an orthonormal tangent frame from a surface normal.
/// Assumes `normal` is non-zero (will be normalized internally).
pub fn local_tangent_frame(normal: Vec3) -> LocalTangentFrame {
    let n = normal.normalized();

    // Choose a reference vector that is not parallel to n
    let reference = if n.z.abs() < 0.9 {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };

    let t1 = reference.cross(n).normalized();
    let t2 = n.cross(t1);

    LocalTangentFrame {
        t1,
        t2,
        normal: n,
    }
}

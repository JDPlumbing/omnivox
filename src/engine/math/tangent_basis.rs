use glam::{Vec3, Mat3};
use crate::core::uvoxid::UvoxId;

pub struct TangentBasis {
    pub east: Vec3,
    pub north: Vec3,
    pub up: Vec3,
    pub matrix: Mat3,
}

pub fn compute_tangent_basis(pos: &UvoxId) -> TangentBasis {
    // 1) Cartesian point in meters
    let (x, y, z) = pos.to_cartesian();
    let p = Vec3::new(x as f32, y as f32, z as f32);

    // 2) Radial up
    let up = p.normalize();

    // 3) Global north direction
    let global_north = Vec3::new(0.0, 0.0, 1.0);

    let mut north = global_north - up * global_north.dot(up);

    if north.length_squared() < 1e-8 {
        north = Vec3::new(1.0, 0.0, 0.0) - up * up.dot(Vec3::new(1.0, 0.0, 0.0));
    }

    north = north.normalize();

    // 4) East = up × north
    let east = up.cross(north).normalize();

    // 5) Construct a basis matrix (east = x, north = y, up = z)
    let matrix = Mat3::from_cols(east, north, up);

    TangentBasis { east, north, up, matrix }
}

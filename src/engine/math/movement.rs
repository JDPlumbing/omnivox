use glam::Vec3;
use crate::core::uvoxid::UvoxId;
use crate::engine::components::quaternion::QuaternionLocal;
use crate::engine::math::orientation::compute_global_orientation;

pub fn compute_movement_vector(
    pos: &UvoxId,
    q_local: &QuaternionLocal,
    forward: f32,
    right: f32,
    up: f32,
) -> Vec3 {
    let q_global = compute_global_orientation(pos, q_local);

    let mut local = Vec3::ZERO;
    local += Vec3::new(0.0, 0.0, -1.0) * forward;
    local += Vec3::new(1.0, 0.0, 0.0) * right;
    local += Vec3::new(0.0, 1.0, 0.0) * up;

    q_global * local
}

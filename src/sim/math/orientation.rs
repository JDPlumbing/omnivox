use glam::{Quat, Mat3};
use crate::core::uvoxid::UvoxId;
use crate::sim::components::quaternion::QuaternionLocal;
use crate::sim::math::tangent_basis::compute_tangent_basis;

pub fn compute_global_orientation(
    pos: &UvoxId,
    q_local: &QuaternionLocal,
) -> Quat {
    let basis = compute_tangent_basis(pos);

    // global basis matrix (curved world frame)
    let q_basis = Quat::from_mat3(&basis.matrix);

    // local rotation (player/camera)
    let q_local = q_local.to_glam();

    // final global orientation
    q_basis * q_local
}

use crate::sim::systems::CameraDelta;
use crate::sim::components::quaternion::QuaternionLocal;
use crate::core::uvoxid::{UvoxId, Delta};
use crate::sim::math::movement::compute_movement_vector;
use crate::sim::math::local_quaternion::apply_local_rotation;

pub fn update_camera_from_delta(
    pos: &mut UvoxId,
    q_local: &mut QuaternionLocal,
    delta: CameraDelta,
) {
    // 1. Update local quaternion from yaw/pitch/roll
    apply_local_rotation(q_local, delta.yaw, delta.pitch, delta.roll);

    // 2. Compute world-space motion vector
    let vec = compute_movement_vector(
        pos,
        q_local,
        delta.move_forward,
        delta.move_right,
        delta.move_up,
    );

    // 3. Convert to Δ(r,lat,lon)
    let d = Delta::from_cartesian_move(pos, vec);

    // 4. Apply curved-world update
    pos.apply_delta(d);
}

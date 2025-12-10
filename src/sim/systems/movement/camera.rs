use crate::sim::systems::CameraDelta;
use crate::sim::components::quaternion::QuaternionLocal;
use crate::sim::math::{
    local_quaternion::apply_local_rotation,
    movement::compute_movement_vector,
};
use crate::core::uvoxid::{UvoxId, Delta};

pub fn apply_camera_delta(
    pos: &mut UvoxId,
    q_local: &mut QuaternionLocal,
    delta: CameraDelta,
) {
    // 1) Orientation update (yaw/pitch/roll)
    apply_local_rotation(q_local, delta.yaw, delta.pitch, delta.roll);

    // 2) Compute the global movement vector in meters
    let move_vec = compute_movement_vector(
        pos,
        q_local,
        delta.move_forward,
        delta.move_right,
        delta.move_up,
    );

    // 3) Convert into global UVOX delta
    let d = Delta::from_cartesian_move(pos, move_vec);

    // 4) Apply curved-world displacement
    pos.apply_delta(d);

    // 5) (Optional later) Update UvoxQuat (global orientation)
}

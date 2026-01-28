
use crate::engine::math::{
    local_quaternion::apply_local_rotation,
    movement::compute_movement_vector,
};
use crate::core::uvoxid::{UvoxId, Delta};
use crate::engine::components::quaternion::QuaternionLocal;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CameraDelta {
    pub yaw: f32,           // radians
    pub pitch: f32,         // radians
    pub roll: f32,          // radians
    pub move_forward: f32,  // meters per tick
    pub move_right: f32,    // meters per tick
    pub move_up: f32,       // meters per tick
}


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

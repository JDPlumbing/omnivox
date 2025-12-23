use glam::Quat;
use crate::engine::components::quaternion::QuaternionLocal;

pub fn apply_local_rotation(
    q_local: &mut QuaternionLocal,
    yaw: f32,
    pitch: f32,
    roll: f32,
) {
    let q = q_local.to_glam();

    // yaw: rotate around local "up" (z+ in tangent frame)
    let q_yaw = Quat::from_rotation_z(yaw);

    // pitch: rotate around local "east/west" axis (x-axis in tangent frame)
    let q_pitch = Quat::from_rotation_x(pitch);

    // roll: rotate around local forward axis (y-axis or -y? depending)
    // Here we choose forward = -Z in tangent frame, so roll is rotation around local Y.
    let q_roll = Quat::from_rotation_y(roll);

    // Combined delta (yaw first, then pitch, then roll)
    let delta = q_yaw * q_pitch * q_roll;

    let new_q = (q * delta).normalize();

    *q_local = QuaternionLocal::from_glam(new_q);
}

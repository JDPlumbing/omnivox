// core/physics/camera/basis.rs

use crate::core::physics::frames::enu::ENUFrame;
use super::pose::CameraPose;

#[derive(Debug, Clone, Copy)]
pub struct CameraBasis {
    pub right:   [f64; 3],
    pub up:      [f64; 3],
    pub forward: [f64; 3],
}

pub fn camera_basis_from_enu(
    enu: ENUFrame,
    pose: CameraPose,
) -> CameraBasis {
    // ----------------------------------------
    // Step 1: yaw rotation in ENU plane
    // ----------------------------------------
    let cy = pose.yaw_rad.cos();
    let sy = pose.yaw_rad.sin();

    let forward_yaw = [
        enu.north[0] * cy + enu.east[0] * sy,
        enu.north[1] * cy + enu.east[1] * sy,
        enu.north[2] * cy + enu.east[2] * sy,
    ];

    let right_yaw = [
        -enu.north[0] * sy + enu.east[0] * cy,
        -enu.north[1] * sy + enu.east[1] * cy,
        -enu.north[2] * sy + enu.east[2] * cy,
    ];

    // ----------------------------------------
    // Step 2: pitch rotation
    // ----------------------------------------
    let cp = pose.pitch_rad.cos();
    let sp = pose.pitch_rad.sin();

    let forward = [
        forward_yaw[0] * cp + enu.up[0] * sp,
        forward_yaw[1] * cp + enu.up[1] * sp,
        forward_yaw[2] * cp + enu.up[2] * sp,
    ];

    let up = [
        -forward_yaw[0] * sp + enu.up[0] * cp,
        -forward_yaw[1] * sp + enu.up[1] * cp,
        -forward_yaw[2] * sp + enu.up[2] * cp,
    ];

    CameraBasis {
        right: right_yaw,
        up,
        forward,
    }
}

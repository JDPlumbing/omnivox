// core/physics/camera/project.rs

use super::basis::CameraBasis;
use crate::core::math::vec3::dot;

#[derive(Debug, Clone, Copy)]
pub struct CameraVector {
    pub x: f64, // right
    pub y: f64, // up
    pub z: f64, // forward
}
#[derive(Debug, Clone, Copy)]
pub struct CameraBodyDisk {
    pub visible: bool,
    pub ndc: Option<[f64; 2]>,
    pub angular_radius_rad: f64,
}

/// Project a world-space direction into camera space
pub fn project_world_dir_to_camera(
    basis: CameraBasis,
    world_dir: [f64; 3],
) -> CameraVector {
    CameraVector {
        x: dot(basis.right, world_dir),
        y: dot(basis.up, world_dir),
        z: dot(basis.forward, world_dir),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CameraProjection {
    /// Vertical field of view in radians
    pub fov_y_rad: f64,

    /// Width / height
    pub aspect: f64,
}

impl CameraProjection {
    /// Horizontal FOV derived from vertical FOV
    pub fn fov_x_rad(&self) -> f64 {
        2.0 * ((self.fov_y_rad * 0.5).tan() * self.aspect).atan()
    }

    /// Check if a camera-space vector is inside the view frustum
    pub fn is_visible(&self, v: CameraVector) -> bool {
        // Must be in front of the camera
        if v.z <= 0.0 {
            return false;
        }

        let half_fov_x = self.fov_x_rad() * 0.5;
        let half_fov_y = self.fov_y_rad * 0.5;

        let az = v.x.atan2(v.z);
        let el = v.y.atan2(v.z);

        az.abs() <= half_fov_x && el.abs() <= half_fov_y
    }

    /// Project to normalized device coordinates [-1, 1]
    pub fn project_to_ndc(&self, v: CameraVector) -> Option<[f64; 2]> {
        if v.z <= 0.0 {
            return None;
        }

        let tan_half_y = (self.fov_y_rad * 0.5).tan();
        let tan_half_x = tan_half_y * self.aspect;

        let x_ndc = v.x / (v.z * tan_half_x);
        let y_ndc = v.y / (v.z * tan_half_y);

        if x_ndc.abs() > 1.0 || y_ndc.abs() > 1.0 {
            None
        } else {
            Some([x_ndc, y_ndc])
        }
    }
}

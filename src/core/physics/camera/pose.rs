// core/physics/camera/pose.rs

#[derive(Debug, Clone, Copy)]
pub struct CameraPose {
    /// Rotation around local Up axis (ENU.up)
    /// 0 = facing North, +π/2 = East
    pub yaw_rad: f64,

    /// Rotation around local Right axis
    /// 0 = level, +π/2 = straight up
    pub pitch_rad: f64,
}

impl CameraPose {
    pub fn forward_only() -> Self {
        Self {
            yaw_rad: 0.0,
            pitch_rad: 0.0,
        }
    }
}

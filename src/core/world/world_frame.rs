use crate::core::id::WorldId;
use crate::core::tdt::{SimTime, SimDuration};
use std::collections::HashMap;
use std::ops::Mul;

use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::uvoxid::UvoxId;

/// Defines how a world is positioned relative to a parent frame
#[derive(Clone)]
pub struct WorldFrame {
    pub world_id: WorldId,

    /// None = root frame (e.g. Sun)
    pub parent: Option<WorldId>,

    /// Computes this world's transform at a given time
    pub model: FrameModel,
}

#[derive(Clone)]
pub enum FrameModel {
    /// Fixed in parent frame
    Static {
        position: UvoxId,
    },

    /// Time-varying translation (orbit only for now)
    Orbital {
        params: OrbitalParams,
    },
}

#[derive(Clone)]
pub struct OrbitalParams {
    /// Orbital radius (meters)
    pub semi_major_axis_m: f64,

    /// Orbital period
    pub period: SimDuration,

    /// Inclination relative to parent frame (radians)
    pub inclination_rad: f64,

    /// Phase offset at epoch (radians)
    pub phase_at_epoch: f64,

    /// Rotation period (sidereal)
    pub rotation_period: SimDuration,

    /// Rotation phase at epoch (radians)
    pub rotation_phase_at_epoch: f64,

    /// Axial tilt relative to orbital plane (radians)
    pub axial_tilt_rad: f64,

}


/// Full rigid-body pose of a frame relative to its parent
#[derive(Clone, Copy)]
pub struct FramePose {
    pub position_m: [f64; 3],
    pub orientation: Mat3, // local → parent
}

/// Simple 3×3 rotation matrix
#[derive(Clone, Copy)]
pub struct Mat3 {
    pub m: [[f64; 3]; 3],
}

impl Mat3 {
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
    pub fn rotation_z(theta: f64) -> Self {
        let c = theta.cos();
        let s = theta.sin();

        Self {
            m: [
                [ c, -s, 0.0],
                [ s,  c, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
    pub fn rotation_x(theta: f64) -> Self {
        let c = theta.cos();
        let s = theta.sin();

        Self {
            m: [
                [1.0, 0.0, 0.0],
                [0.0,  c, -s],
                [0.0,  s,  c],
            ],
        }
    }
}

/// Matrix × vector
impl Mul<[f64; 3]> for Mat3 {
    type Output = [f64; 3];

    fn mul(self, v: [f64; 3]) -> [f64; 3] {
        [
            self.m[0][0] * v[0] + self.m[0][1] * v[1] + self.m[0][2] * v[2],
            self.m[1][0] * v[0] + self.m[1][1] * v[1] + self.m[1][2] * v[2],
            self.m[2][0] * v[0] + self.m[2][1] * v[1] + self.m[2][2] * v[2],
        ]
    }
}

/// Matrix × matrix
impl Mul for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Mat3 {
        let mut r = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    r[i][j] += self.m[i][k] * rhs.m[k][j];
                }
            }
        }
        Mat3 { m: r }
    }
}

impl FrameModel {
    pub fn pose_at(&self, time: SimTime) -> FramePose {
        match self {
            FrameModel::Static { position } => {
                let v = position.to_vec3();

                FramePose {
                    position_m: [v[0] as f64, v[1] as f64, v[2] as f64],
                    orientation: Mat3::identity(),
                }
            }


            FrameModel::Orbital { params } => {
                let t_ns = time.0 as f64;
                let period_ns = params.period.0 as f64;

                // Orbital angle
                let theta =
                    2.0 * std::f64::consts::PI * (t_ns / period_ns)
                    + params.phase_at_epoch;

                let r = params.semi_major_axis_m;

                let x = r * theta.cos();
                let y_raw = r * theta.sin();

                let z = y_raw * params.inclination_rad.sin();
                let y = y_raw * params.inclination_rad.cos();

                // Spin (sidereal rotation)
                let spin_theta =
                    2.0 * std::f64::consts::PI
                    * (t_ns / params.rotation_period.0 as f64)
                    + params.rotation_phase_at_epoch;

                let spin = Mat3::rotation_z(spin_theta);

                // Axial tilt (constant)
                let tilt = Mat3::rotation_x(params.axial_tilt_rad);

                let orientation = spin * tilt;

                FramePose {
                    position_m: [x, y, z],
                    orientation,
                }
            }


        }
    }
}

pub struct WorldResolver<'a> {
    pub frames: &'a HashMap<WorldId, WorldFrame>,
}

impl<'a> WorldResolver<'a> {
    pub fn world_pose(
        &self,
        world_id: WorldId,
        time: SimTime,
    ) -> FramePose {
        let frame = self.frames
            .get(&world_id)
            .expect("missing world frame");

        let local_pose = frame.model.pose_at(time);

        if let Some(parent) = frame.parent {
            let parent_pose = self.world_pose(parent, time);

            FramePose {
                position_m: {
                    let rotated = parent_pose.orientation * local_pose.position_m;
                    [
                        parent_pose.position_m[0] + rotated[0],
                        parent_pose.position_m[1] + rotated[1],
                        parent_pose.position_m[2] + rotated[2],
                    ]
                },
                orientation: parent_pose.orientation * local_pose.orientation,
            }
        } else {
            local_pose
        }
    }

    pub fn world_point(
        &self,
        world_id: WorldId,
        uvox: &UvoxId,
        time: SimTime,
        space: &WorldSpace,
    ) -> [f64; 3] {
        let pose = self.world_pose(world_id, time);
        let local = uvox_local_offset_m(uvox, space);
        let rotated = pose.orientation * local;

        [
            pose.position_m[0] + rotated[0],
            pose.position_m[1] + rotated[1],
            pose.position_m[2] + rotated[2],
        ]
    }
}

pub fn uvox_local_offset_m(
    uvox: &UvoxId,
    space: &WorldSpace,
) -> [f64; 3] {
    let p = uvox.to_vec3();
    let pos = [p[0] as f64, p[1] as f64, p[2] as f64];

    let r = uvox.r_um.meters();
    let surface = space.surface_radius_m;

    let scale = (r - surface) / r.max(1.0);

    [
        pos[0] * scale,
        pos[1] * scale,
        pos[2] * scale,
    ]
}

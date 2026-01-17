use crate::core::id::WorldId;
use crate::core::tdt::SimTime;
use crate::core::tdt::SimDuration;
use std::collections::HashMap;

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

    /// Time-varying (orbits, rotation, etc.)
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
}
impl FrameModel {
pub fn position_at(&self, time: SimTime) -> [f64; 3] {
    match self {
    FrameModel::Static { position } => {
        let v = position.to_vec3(); // [f32; 3]
        [v[0] as f64, v[1] as f64, v[2] as f64]
    }


        FrameModel::Orbital { params } => {
            let t_ns = time.0 as f64;
            let period_ns = params.period.0 as f64;

            let theta =
                2.0 * std::f64::consts::PI * (t_ns / period_ns)
                + params.phase_at_epoch;

            let r = params.semi_major_axis_m;

            let x = r * theta.cos();
            let y = r * theta.sin();

            let z = y * params.inclination_rad.sin();
            let y = y * params.inclination_rad.cos();

            [x, y, z]
        }
    }
}

}

pub struct WorldResolver<'a> {
    pub frames: &'a HashMap<WorldId, WorldFrame>,
}

impl<'a> WorldResolver<'a> {
    pub fn world_origin(
        &self,
        world_id: WorldId,
        time: SimTime,
    ) -> [f64; 3] {
        let frame = self.frames.get(&world_id)
            .expect("missing world frame");

        let local = frame.model.position_at(time);

        if let Some(parent) = frame.parent {
            let parent_pos = self.world_origin(parent, time);
            [
                parent_pos[0] + local[0],
                parent_pos[1] + local[1],
                parent_pos[2] + local[2],
            ]
        } else {
            local
        }
    }
}

impl<'a> WorldResolver<'a> {
    pub fn world_point(
        &self,
        world_id: WorldId,
        uvox: &UvoxId,
        time: SimTime,
        space: &WorldSpace,
    ) -> [f64; 3] {
        let origin = self.world_origin(world_id, time);
        let local = uvox_local_offset_m(uvox, space);

        [
            origin[0] + local[0],
            origin[1] + local[1],
            origin[2] + local[2],
        ]
    }
}


pub fn uvox_local_offset_m(
    uvox: &UvoxId,
    space: &WorldSpace,
) -> [f64; 3] {
    let p = uvox.to_vec3(); // [f32; 3]
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

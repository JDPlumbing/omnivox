use crate::core::id::WorldId;
use crate::core::tdt::SimTime;
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::world::world_frame::WorldResolver;

use super::enu::ENUFrame;
use crate::core::physics::tides::AnchorError;

#[derive(Debug)]
pub enum FrameError {
    SingularAnchor,
}

impl From<AnchorError> for FrameError {
    fn from(_: AnchorError) -> Self {
        FrameError::SingularAnchor
    }
}

/// Local tangent frame at a physical anchor point
#[derive(Debug, Clone, Copy)]
pub struct LocalTangentFrame {
    pub origin: [f64; 3], // world-space position
    pub enu: ENUFrame,
}

pub fn local_tangent_frame(
    resolver: &WorldResolver,
    world: WorldId,
    anchor: &UvoxId,
    time: SimTime,
    space: &WorldSpace,
) -> Result<LocalTangentFrame, FrameError> {
    // --------------------------------------------
    // Resolve anchor point (rejects r = 0)
    // --------------------------------------------
    let origin = resolver
        .world_anchor_point(world, anchor, time, space)
        .map_err(|_| FrameError::SingularAnchor)?;

    // --------------------------------------------
    // ENU axes defined at anchor
    // --------------------------------------------
    let enu = super::enu::enu_frame(
        resolver,
        world,
        anchor,
        time,
        space,
    )?;

    Ok(LocalTangentFrame {
        origin,
        enu,
    })
}

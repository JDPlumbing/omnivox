use crate::core::math::vec3::Vec3;
use crate::core::physics::units::irradiance::WattsPerSquareMeter;
use crate::core::worlds::id::WorldId;
use crate::core::worlds::state::WorldState;
use crate::core::cosmic::state::CosmicState;
use crate::core::cosmic::systems::frame_system::CosmicFrameSystem;
use crate::core::cosmic::systems::radiation_system::CosmicRadiationSystem;
use crate::core::tdt::sim_time::SimTime;

/// Radiation arriving at a world surface, expressed in the
/// world-body local frame.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceRadiation {
    pub direction_local: Vec3,
    pub flux: WattsPerSquareMeter,
}

pub fn radiation_at_surface(
    world_id: WorldId,
    world_state: &WorldState,
    cosmic_state: &CosmicState,
    time: SimTime,
) -> Option<SurfaceRadiation> {
    let anchor = world_state.anchors.get(&world_id)?;
    let body_id = anchor.body;

    let frames = CosmicFrameSystem { state: cosmic_state };
    let radiation_system = CosmicRadiationSystem {
        state: cosmic_state,
        frames: &frames,
    };

    // Deterministic primary star (v1 assumption)
    let orbit = cosmic_state.orbits.get(&body_id)?;
    let star_id = orbit.primary;

    let radiation =
        radiation_system.radiation_from_body(star_id, body_id, time)?;

    let body_pose = frames.body_pose(body_id, time);

    // Direction from surface → star, in body-local frame
    let direction_local =
        body_pose.orientation.inverse() * (-radiation.direction);

    Some(SurfaceRadiation {
        direction_local,
        flux: radiation.flux,
    })
}

use crate::core::math::frames::enu::enu_frame;
use crate::core::math::vec3::Vec3;

use crate::core::worlds::id::WorldId;
use crate::core::worlds::state::WorldState;

use crate::core::cosmic::state::CosmicState;
use crate::core::cosmic::systems::frame_system::CosmicFrameSystem;

use crate::core::tdt::sim_time::SimTime;

use crate::core::spatial::surface_coords::SurfaceCoords;



/// Resolve a point on a world's surface into cosmic-space position.
pub fn world_surface_position_cosmic(
    world_id: WorldId,
    world_state: &WorldState,
    cosmic_state: &CosmicState,
    surface: SurfaceCoords,
    time: SimTime,
) -> Vec3 {
    // 1️⃣ Resolve world anchor → cosmic body
    let anchor = world_state.anchors
        .get(&world_id)
        .expect("world has no anchor");

    let body_id = anchor.body;

    // 2️⃣ Resolve cosmic body pose
    let frames = CosmicFrameSystem { state: cosmic_state };
    let body_pose = frames.body_pose(body_id, time);

    let center = body_pose.position;

    // 3️⃣ Base cosmic body radius (meters)
    let base_radius_m = cosmic_state
        .radii
        .get(&body_id)
        .expect("cosmic body has no radius")
        .meters
        .0;

    // 4️⃣ Effective radius = base + elevation
    let radius_m = base_radius_m + surface.elevation.0;

    // 5️⃣ Surface normal in body-local frame
    let lat = surface.latitude.0;
    let lon = surface.longitude.0;

    let local_normal = Vec3::new(
        lat.cos() * lon.cos(),
        lat.cos() * lon.sin(),
        lat.sin(),
    );

    // 6️⃣ Rotate normal into cosmic orientation
    let rotated_normal = body_pose.orientation * local_normal;

    // 7️⃣ Final cosmic position
    center + rotated_normal * radius_m
}

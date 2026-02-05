use crate::core::worlds::id::WorldId;
use crate::core::worlds::state::WorldState;
use crate::core::spatial::location::Location;
use crate::core::cosmic::state::CosmicState;
use crate::core::cosmic::systems::frame_system::CosmicFrameSystem;
use crate::core::cosmic::systems::gravity_point::acceleration_vector_at_point_from_body;
use crate::core::math::frames::enu::enu_frame;
use crate::core::math::vec3::Vec3;
use crate::core::tdt::sim_time::SimTime;
use crate::core::physics::units::acceleration::MetersPerSecondSquared;

#[derive(Debug, Clone, Copy)]
pub struct LocalENU {
    pub east: MetersPerSecondSquared,
    pub north: MetersPerSecondSquared,
    pub up: MetersPerSecondSquared,
}

pub fn gravity_enu_at_location(
    world_id: WorldId,
    location: &Location,
    world_state: &WorldState,
    cosmic_state: &CosmicState,
    time: SimTime,
) -> LocalENU {
    // 1️⃣ Resolve world → cosmic body
    let anchor = world_state.anchors
        .get(&world_id)
        .expect("world has no anchor");

    let body_id = anchor.body;

    // 2️⃣ Cosmic frame
    let frames = CosmicFrameSystem { state: cosmic_state };
    let body_pose = frames.body_pose(body_id, time);
    let body_center = body_pose.position;

    // 3️⃣ Surface normal (body-local)
    let lat = location.latitude.0;
    let lon = location.longitude.0;

    let local_normal = Vec3::new(
        lat.cos() * lon.cos(),
        lat.cos() * lon.sin(),
        lat.sin(),
    );

    // World north pole direction in cosmic frame
    let north_hint = body_pose.orientation * Vec3::new(0.0, 0.0, 1.0);

    // Rotate normal into cosmic frame
    let up = body_pose.orientation * local_normal;

    // 4️⃣ Surface position in cosmic space
    let radius = cosmic_state.radii[&body_id].meters.0;
    let surface_pos = body_center + up * radius;

    // 5️⃣ Cosmic gravity vector at that point
    let g_cosmic: Vec3 = acceleration_vector_at_point_from_body(
        cosmic_state,
        body_id,
        surface_pos,
        body_center,
    );

    // 6️⃣ Build ENU basis
    let enu = enu_frame(up, north_hint);

    // 7️⃣ Project gravity into ENU
    LocalENU {
        east: MetersPerSecondSquared(g_cosmic.dot(enu.east)),
        north: MetersPerSecondSquared(g_cosmic.dot(enu.north)),
        up: MetersPerSecondSquared(g_cosmic.dot(enu.up)),
    }
}


pub fn gravity_at_surface(
    world_id: WorldId,
    location: &Location,
    world_state: &WorldState,
    cosmic_state: &CosmicState,
    time: SimTime,
) -> MetersPerSecondSquared {
    let g = gravity_enu_at_location(
        world_id,
        location,
        world_state,
        cosmic_state,
        time,
    );

    // Up is negative (toward center), so negate
    MetersPerSecondSquared(-g.up.0)
}

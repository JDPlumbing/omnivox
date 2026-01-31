use crate::core::entity::components::spatial::Location;
use crate::core::worlds::systems::frame_mapping::world_surface_position_cosmic;
use crate::core::math::vec3::magnitude;
use crate::core::tdt::sim_time::SimTime;
use crate::core::cosmic::id::CosmicBodyId;
use crate::core::worlds::id::WorldId;
use crate::core::worlds::components::world_anchor::WorldAnchor;
use crate::core::worlds::state::WorldState;
use crate::core::cosmic::state::CosmicState;
use crate::core::cosmic::components::radius::Radius;
use crate::core::cosmic::components::mass::Mass;
use crate::core::cosmic::components::root::Root;
use crate::core::physics::units::length::Meters;
use crate::core::physics::units::mass::Kilograms;
use crate::core::cosmic::systems::frame_system::CosmicFrameSystem;

#[test]
pub fn surface_point_is_one_radius_from_center() {
    // Arrange
    let time = SimTime::from_ns(0);

    let earth_body = CosmicBodyId(1);
    let earth_world = WorldId(1);

    let cosmic_state = test_earth_cosmic_state(earth_body);
    let world_state = test_world_anchored_to_body(earth_world, earth_body);

    let location = Location {
        latitude: 0.0.into(),   // equator
        longitude: 0.0.into(),  // prime meridian
    };

    // Act
    let surface_pos = world_surface_position_cosmic(
        earth_world,
        &world_state,
        &cosmic_state,
        &location,
        time,
    );

    let frames = CosmicFrameSystem { state: &cosmic_state };
    let center = frames.body_pose(earth_body, time).position_m;

    let r = magnitude([
        surface_pos[0] - center[0],
        surface_pos[1] - center[1],
        surface_pos[2] - center[2],
    ]);

    let earth_radius = cosmic_state.radii[&earth_body].meters.0;

    // Assert
    assert!(
        (r - earth_radius).abs() < 1.0,
        "expected surface radius {}, got {}",
        earth_radius,
        r
    );
}

// ------------------------
// Test helpers
// ------------------------

pub fn test_earth_cosmic_state(earth: CosmicBodyId) -> CosmicState {
    let mut state = CosmicState::default();

    state.radii.insert(
        earth,
        Radius { meters: Meters(6_371_000.0) },
    );

    state.masses.insert(
        earth,
        Mass { kg: Kilograms(5.972e24) },
    );

    state.roots.insert(earth, Root);

    state
}

pub fn test_world_anchored_to_body(
    world: WorldId,
    body: CosmicBodyId,
) -> WorldState {
    let mut state = WorldState::default();

    state.anchors.insert(
        world,
        WorldAnchor {
            body: body,
        },
    );

    state
}

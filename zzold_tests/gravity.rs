use crate::core::tdt::sim_time::SimTime;
use crate::core::cosmic::id::CosmicBodyId;
use crate::core::worlds::id::WorldId;
use crate::core::entity::components::spatial::Location;
use crate::core::worlds::systems::gravity::gravity_enu_at_location;
use crate::tests::surface_point::test_world_anchored_to_body;
use crate::tests::surface_point::test_earth_cosmic_state;



#[test]
fn gravity_points_down_at_surface() {
    let time = SimTime::from_ns(0);

    let earth_body = CosmicBodyId(1);
    let earth_world = WorldId(1);

    let cosmic_state = test_earth_cosmic_state(earth_body);
    let world_state = test_world_anchored_to_body(earth_world, earth_body);

    let location = Location {
        latitude: 0.0.into(),
        longitude: 0.0.into(),
    };

    let g = gravity_enu_at_location(
        earth_world,
        &location,
        &world_state,
        &cosmic_state,
        time,
    );

    assert!(g.up < 0.0, "gravity should point downward");
    assert!(g.east.abs() < 1e-6);
    assert!(g.north.abs() < 1e-6);
}

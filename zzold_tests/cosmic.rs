// core/cosmic/systems/tests.rs

use crate::core::cosmic::{
    id::CosmicBodyId,
    state::CosmicState,
    components::{
        mass::Mass,
        radius::Radius,
        luminosity::Luminosity,
        orbit::Orbit,
        rotation::Rotation,
        root::Root,
    },
};
use crate::core::physics::units::{
    length::Meters,
    mass::Kilograms,
    power::Watts,
    angle::Radians,
    time::Seconds,
};
use crate::core::tdt::SimTime;

const SUN: CosmicBodyId = CosmicBodyId(0);
const EARTH: CosmicBodyId = CosmicBodyId(1);

fn build_simple_solar_system() -> CosmicState {
    let mut state = CosmicState::default();

    // Sun
    state.roots.insert(SUN, Root);
    state.masses.insert(SUN, Mass { kg: Kilograms(1.98847e30) });
    state.radii.insert(SUN, Radius { meters: Meters(6.9634e8) });
    state.luminosities.insert(SUN, Luminosity { watts: Watts(3.828e26) });

    // Earth
    state.orbits.insert(EARTH, Orbit {
        primary: SUN,
        semi_major_axis: Meters(1.495978707e11), // 1 AU
        period: Seconds(31_557_600.0),           // 1 year
        inclination: Radians(0.0),
        phase_at_epoch: 0.0.into(),
    });

    state.masses.insert(EARTH, Mass { kg: Kilograms(5.972e24) });
    state.radii.insert(EARTH, Radius { meters: Meters(6.371e6) });

    state.rotations.insert(EARTH, Rotation {
        period: Seconds(86_400.0),
        phase_at_epoch: 0.0.into(),
    });

    state
}
use crate::core::cosmic::systems::frame_system::CosmicFrameSystem;

#[test]
fn earth_is_about_one_au_from_sun() {
    let state = build_simple_solar_system();
    let frames = CosmicFrameSystem { state: &state };

    let t = SimTime(0);

    let sun_pose = frames.body_pose(SUN, t);
    let earth_pose = frames.body_pose(EARTH, t);

    let dx = earth_pose.position_m[0] - sun_pose.position_m[0];
    let dy = earth_pose.position_m[1] - sun_pose.position_m[1];
    let dz = earth_pose.position_m[2] - sun_pose.position_m[2];

    let dist = (dx*dx + dy*dy + dz*dz).sqrt();

    assert!(
        (dist - 1.496e11).abs() < 1.0e9,
        "Earth distance {} m", dist
    );
}
use crate::core::cosmic::systems::radiation_system::CosmicRadiationSystem;

#[test]
fn solar_flux_at_earth_is_reasonable() {
    let state = build_simple_solar_system();
    let frames = CosmicFrameSystem { state: &state };
    let radiation = CosmicRadiationSystem {
        state: &state,
        frames: &frames,
    };

    let t = SimTime(0);

    let sample = radiation
        .radiation_from_body(SUN, EARTH, t)
        .expect("Sun emits radiation");

    // Expected ~1361 W/m²
    assert!(
        (sample.flux_w_m2 - 1361.0).abs() < 100.0,
        "Flux = {} W/m²", sample.flux_w_m2
    );
}
use crate::core::cosmic::systems::gravity_system::CosmicGravitySystem;

#[test]
fn earth_gravity_points_toward_sun() {
    let state = build_simple_solar_system();
    let frames = CosmicFrameSystem { state: &state };
    let gravity = CosmicGravitySystem {
        state: &state,
        frames: &frames,
    };

    let t = SimTime(0);

    let accel = gravity.total_acceleration(EARTH, t).accel_m_s2;

    // Should point roughly toward Sun (negative X)
    assert!(accel[0] < 0.0, "Accel = {:?}", accel);
}

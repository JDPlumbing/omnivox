// This is a golden end-to-end test.
// It validates Uvox → Surface → World → Cosmic → Environment → Entity consistency.
// If this test fails, a fundamental layer boundary is broken.


use crate::core::simulation::sim_engine::SimulationEngine;
use crate::core::tdt::SimTime;
use crate::shared::EntityStore;
use crate::core::worlds::id::WorldId;
use crate::core::entity::id::EntityId;
use crate::core::spatial::surface::SurfaceCoords;
use crate::core::physics::units::angle::Radians;
use crate::core::spatial::resolve::surface_to_uvox;
use crate::core::entity::components::position::Position;
use crate::core::entity::components::position_enu::PositionENU;
use crate::core::entity::components::velocity_enu::VelocityENU;
use crate::core::entity::components::grounded::Grounded;
use crate::core::entity::components::world_membership::WorldMembership;
use crate::core::worlds::components::world_surface::WorldSurface;
use crate::core::worlds::state::WorldState;
use crate::core::cosmic::id::CosmicBodyId;
use crate::core::physics::units::length::Meters;
use crate::core::worlds::components::world_anchor::WorldAnchor;
use crate::core::physics::units::velocity::MetersPerSecond;
use crate::core::cosmic::components::orbit::Orbit;
use crate::core::cosmic::components::rotation::Rotation;
use crate::core::cosmic::components::axial_tilt::AxialTilt;
use crate::core::cosmic::components::prime_meridian::PrimeMeridian;
use crate::core::cosmic::state::CosmicState;
use crate::core::cosmic::components::root::Root;
use crate::core::cosmic::components::luminosity::Luminosity;
use crate::core::physics::units::power::Watts;
use crate::core::cosmic::components::radius::Radius;
use crate::core::physics::units::time::Seconds;
use crate::core::cosmic::components::mass::Mass;
use crate::core::physics::units::mass::Kilograms;
use crate::tests::fixtures::cosmic::simple_star_planet;


/*
fn setup_simple_star_planet() -> (CosmicState, CosmicBodyId, CosmicBodyId) {
    let mut cosmic = CosmicState::default();

    let star   = CosmicBodyId(1);
    let planet = CosmicBodyId(2);

    // 🌞 Star is a root (no orbit)
    cosmic.roots.insert(star, Root {});
    cosmic.masses.insert(
        star,
        Mass {
            kg: Kilograms(1.989e30), // Sun mass
        },
    );
    cosmic.radii.insert(
        star,
            Radius {
            meters: Meters(1.0),
        },
    );
    cosmic.luminosities.insert(
        star,
        Luminosity {
            watts: Watts(3.828e26), // Sun luminosity
        },
    );

    // 🌍 Planet orbits star in equatorial plane
    cosmic.radii.insert(
        planet,
        Radius {
            meters: Meters(6_371_000.0),
        },
    );  
    cosmic.masses.insert(
        planet,
        Mass {
            kg: Kilograms(5.972e24), // Earth mass
        }
    );
    cosmic.orbits.insert(
        planet,
        Orbit {
            primary: star,
            semi_major_axis: Meters(1.0),
            period: Seconds(365.0 * 86_400.0), // doesn't matter for geometry
            inclination: Radians(0.0),
            phase_at_epoch: Radians(0.0),
        },
    );

    // 🌀 Planet rotation (24h)
    cosmic.rotations.insert(
        planet,
        Rotation {
            period: Seconds(86_400.0),
            phase_at_epoch: Radians(0.0),
        },
    );

    // 🌍 No axial tilt (equatorial sanity test)
    cosmic.axial_tilts.insert(
        planet,
        AxialTilt {
            radians: Radians(0.0),
        },
    );

    // 📍 Prime meridian aligned at epoch
    cosmic.prime_meridians.insert(
        planet,
        PrimeMeridian {
            radians: Radians(0.0),
        },
    );

    (cosmic, star, planet)
}
*/

fn setup_simple_world() -> WorldState {
    let mut world = WorldState::default();
    let world_id = WorldId(1);
    let planet_id = CosmicBodyId(2); // from setup_simple_star_planet
    world.anchors.insert(
        world_id,
        WorldAnchor { body: planet_id }
    );

    world.surfaces.insert(
        world_id,
        WorldSurface::Spherical {
            radius: Meters(6_371_000.0),
        }
    );
    world
    }

#[test]
fn enu_surface_environment_are_consistent() {
    // --- Setup simulation ---
    let mut engine = SimulationEngine::new_with_state(
        SimTime::from_seconds_f64(0.0),
        60_000_000_000, // 60s tick
        simple_star_planet().cosmic,   // star + planet
        setup_simple_world(),    // spherical world
        EntityStore::default(),
    );

    let world_id = WorldId(1);
    let entity = EntityId::new();

    // --- Place entity at equator, prime meridian ---
    let surface = SurfaceCoords::on_surface(
        Radians(0.0),
        Radians(0.0),
    );

    let uvox = surface_to_uvox(
        world_id,
        surface,
        &engine.state.world,
        &engine.state.cosmic,
    );

    engine.state.entities.add_position(entity, Position(uvox));
    engine.state.entities.add_position_enu(
        entity,
        PositionENU {
            east: Meters(0.0),
            north: Meters(0.0),
            up: Meters(0.0),
        },
    );

    engine.state.entities.add_velocity_enu(
        entity,
        VelocityENU {
            east: MetersPerSecond(0.0),
            north: MetersPerSecond(0.0),
            up: MetersPerSecond(0.0),
        },
    );

    engine.state.entities.add_grounded(entity, Grounded);
    engine.state.entities.add_world_membership(
        entity,
        WorldMembership { world_id },
    );
    engine.state.entities.add_active(entity);

    // --- Run one tick ---
    engine.tick();

    // --- Fetch resolved data ---
    let env_sample = engine.state.entities
        .entity_environment_samples
        .get(&entity)
        .expect("entity has no environment sample");

    let accel = engine.state.entities
        .acceleration_enus
        .get(&entity)
        .expect("entity has no acceleration");

    // --- Checks ---

    // 1️⃣ Gravity points downward
    assert!(
        accel.up.0 < 0.0,
        "Gravity should point downward, got {}",
        accel.up.0
    );

    // 2️⃣ Insolation is sane
    assert!(
        env_sample.env.insolation.0 >= 0.0,
        "Insolation should never be negative"
    );

    assert!(
        env_sample.env.insolation.0 >= 0.0,
        "Insolation should never be negative"
    );


}

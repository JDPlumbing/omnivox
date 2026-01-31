#[test]
fn entity_falls_under_gravity() {
    use std::time::Instant;
    let start = Instant::now();
    use crate::core::simulation::sim_engine::SimulationEngine;
    use crate::core::entity::components::spatial::position_enu::PositionENU;
    use crate::core::entity::components::spatial::velocity_enu::VelocityENU;
    use crate::core::physics::units::length::Meters;
    use crate::core::tdt::sim_time::SimTime;
    use crate::core::worlds::id::WorldId;
    use crate::core::entity::id::EntityId;
    use crate::core::physics::units::velocity::MetersPerSecond;
    //use crate::core::physics::units::acceleration::MetersPerSecondSquared;
    //use crate::core::entity::systems::environment_bridge::declare_entity_environment_sample;
    //use crate::core::environment::conditions::EnvironmentConditions;
    use crate::shared::entities::entity_store::EntityStore;
    use crate::core::worlds::components::world_anchor::WorldAnchor;
    use crate::core::cosmic::id::CosmicBodyId;
    use crate::core::cosmic::components::radius::Radius;
    use crate::core::entity::components::active::Active;
    use crate::core::entity::components::spatial::world_membership::WorldMembership;
    use crate::core::physics::units::time::Seconds;
    use crate::core::cosmic::components::mass::Mass;
    use crate::core::physics::units::mass::Kilograms;
    use crate::core::cosmic::components::luminosity::Luminosity;
    use crate::core::physics::units::power::Watts;
    use crate::core::cosmic::components::orbit::Orbit;
    use crate::core::cosmic::components::root::Root;

    use crate::core::cosmic::components::axial_tilt::AxialTilt;
    use crate::core::cosmic::components::rotation::Rotation;
    use crate::core::physics::units::angle::Radians;
    use crate::core::worlds::components::world_surface::WorldSurface;


    // 1️⃣ Create simulation
    let mut sim = SimulationEngine::new(
        /*time=*/SimTime::from_ns(0),
        /*tick_delta_ns=*/3_600_000_000_000, //  1 hour 
        /*entities=*/EntityStore::default(),
    );
    
    // 2️⃣ Create a world (assumes you already have helpers for this)

    let world_id = WorldId(1);
    let earth_id = CosmicBodyId(1);
    let sun_id = CosmicBodyId(2);
    //make the sun
    sim.state.cosmic.radii.insert(
        sun_id,
        Radius {
            meters: Meters(696_340_000.0),
        },
    );
    //give it luminosity    
    sim.state.cosmic.luminosities.insert(
        sun_id,
        Luminosity { watts: Watts(3.828e26) } // Sun
    );
    sim.state.cosmic.masses.insert(
        sun_id,
        Mass {
            kg: Kilograms(1.989e30), // Sun mass
        },
    );
    sim.state.cosmic.roots.insert(
        sun_id,
        Root {},
    );
    sim.state.cosmic.orbits.insert(
        earth_id,
        Orbit {
            primary: sun_id,
            semi_major_axis: Meters(149_597_870_700.0), // 1 AU
            period: Seconds(31_556_952.0),               // 1 year
            inclination: Radians(0.0),
            phase_at_epoch: Radians(0.0),
        },
    );

    sim.state.cosmic.rotations.insert(
        earth_id,
        Rotation {
            period: Seconds(86_400.0), // 1 Earth day
            phase_at_epoch: Radians(0.0),
        },
    );
    sim.state.cosmic.axial_tilts.insert(
        earth_id,
        AxialTilt {
            radians: Radians(23.439281 * std::f64::consts::PI / 180.0), // Earth tilt
        },
    );
    // Earth-like radius is fine
    sim.state.cosmic.radii.insert(
        earth_id,
        Radius {
            meters: Meters(6_371_000.0),
        },
    );

    sim.state.cosmic.masses.insert(
        earth_id,
        Mass {
            kg: Kilograms(5.972e24), // Earth mass
        },
    );
    //now crreate worlds
    sim.state.world.anchors.insert(
        world_id,
        WorldAnchor { body: earth_id },
    );

    // After creating world_id
    sim.state.world.surfaces.insert(
        world_id,
    WorldSurface::default(),

    );


    // 3️⃣ Spawn entity
    let entity = EntityId::new();


    sim.state.entities.actives.insert(entity, Active);

    
    sim.state.entities.world_memberships.insert(
        entity,
        WorldMembership { world_id },
    );


    sim.state.entities.position_enus.insert(
        entity,
        PositionENU {
            east: Meters(0.0),
            north: Meters(0.0),
            up: Meters(10.0), // start 10m above surface
        },
    );

    sim.state.entities.velocity_enus.insert(
        entity,
        VelocityENU {
            east: MetersPerSecond(0.0),
            north: MetersPerSecond(0.0),
            up: MetersPerSecond(0.0),
        },
    );
    let initial_exposure = sim
        .state
        .entities
        .exposures
        .get(&entity)
        .map(|e| e.radiant.0)
        .unwrap_or(0.0);

    let mut ever_grounded = false;

   // 4️⃣ Tick simulation
let initial_up = sim.state.entities.position_enus[&entity].up.0;
let initial_exposure = sim
    .state
    .entities
    .exposures
    .get(&entity)
    .map(|e| e.radiant.0)
    .unwrap_or(0.0);

let mut ever_grounded = false;
let total_ticks = 24; // 10 (fall) + 48 (half-day)

// Run simulation
for _ in 0..total_ticks {
    sim.tick();

    if sim.state.entities.groundeds.contains_key(&entity) {
        ever_grounded = true;
    }
}

// Sample final state
let final_up = sim.state.entities.position_enus[&entity].up.0;
let final_exposure = sim
    .state
    .entities
    .exposures
    .get(&entity)
    .map(|e| e.radiant.0)
    .unwrap_or(0.0);

// Debug output (honest)
let elapsed = start.elapsed();
println!(
    "Simulated {} ticks in {:?} ({:.3} µs per tick)",
    total_ticks,
    elapsed,
    elapsed.as_secs_f64() * 1e6 / total_ticks as f64
);

println!(
    "entity {:?} up = {:.3}, grounded = {}",
    entity,
    final_up,
    ever_grounded
);

// 5️⃣ Assertions
assert!(
    final_up < initial_up,
    "Entity did not fall: initial_up={}, final_up={}",
    initial_up,
    final_up
);

assert!(
    ever_grounded,
    "Entity never became grounded"
);

assert!(
    final_exposure > initial_exposure,
    "Exposure did not accumulate: initial={}, final={}",
    initial_exposure,
    final_exposure
);
}

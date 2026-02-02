use crate::shared::entities::entity_store::EntityStore;
use crate::core::entity::id::EntityId;
use crate::core::entity::components::{
    geometry_parts::Radius,
    materials::Density,
    entity_environment_sample::EntityEnvironmentSample,
};
use crate::core::entity::systems::{
    compute_entity_mass::compute_entity_mass,
    compute_entity_weight::compute_entity_weight,
};
use crate::core::worlds::systems::gravity::LocalENU;
use crate::core::physics::units::{
    length::Meters,
    density::KilogramsPerCubicMeter,
    acceleration::MetersPerSecondSquared,
};
use crate::core::environment::conditions::EnvironmentConditions;
use crate::core::entity::components::active::Active;

#[test]
fn entity_weight_scales_with_gravity() {
    let mut store = EntityStore::default();
    let entity = EntityId::new();

    // --- Geometry & material ---
    store.add_radius(entity, Radius(Meters(1.0)));
    store.add_density(
        entity,
        Density(KilogramsPerCubicMeter(1000.0)),
    );

    store.actives.insert(entity, Active);

    // --- Compute mass ---
    compute_entity_mass(&mut store);

    let mass = store.masses
        .get(&entity)
        .expect("mass should exist")
        .0 .0;

    // --- Earth gravity ---
    let gravity_earth = LocalENU {
        east: MetersPerSecondSquared(0.0),
        north: MetersPerSecondSquared(0.0),
        up: MetersPerSecondSquared(-9.81),
    };

    store.entity_environment_samples.insert(
        entity,
        EntityEnvironmentSample {
            env: EnvironmentConditions::default(),
            gravity_enu: gravity_earth,
        },
    );

    compute_entity_weight(&mut store);

    let weight_earth = store.weights
        .get(&entity)
        .expect("earth weight")
        .0
        .0;

    // --- Moon gravity ---
    let gravity_moon = LocalENU {
        east: MetersPerSecondSquared(0.0),
        north: MetersPerSecondSquared(0.0),
        up: MetersPerSecondSquared(-1.62),
    };

    store.entity_environment_samples.insert(
        entity,
        EntityEnvironmentSample {
            env: EnvironmentConditions::default(),
            gravity_enu: gravity_moon,
        },
    );

    compute_entity_weight(&mut store);

    let weight_moon = store.weights
        .get(&entity)
        .expect("moon weight")
        .0
        .0;

    // --- Assertions ---

    // Weight must scale linearly with gravity
    let ratio = weight_earth.abs() / weight_moon.abs();

    assert!(
        (ratio - (9.81 / 1.62)).abs() < 0.01,
        "weight ratio incorrect: {}",
        ratio
    );

    // Direction must be downward
    assert!(weight_earth < 0.0);
    assert!(weight_moon < 0.0);

    // Sanity: |weight| ≈ m * g
    assert!(
        (weight_earth.abs() - mass * 9.81).abs() < 1.0,
        "weight magnitude incorrect"
    );
}

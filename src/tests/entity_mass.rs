use crate::shared::entities::entity_store::EntityStore;
use crate::core::entity::id::EntityId;
use crate::core::entity::components::{
    geometry_parts::Radius,
    materials::Density,
};
use crate::core::entity::systems::compute_entity_mass::compute_entity_mass;
use crate::core::physics::units::{
    length::Meters,
    density::KilogramsPerCubicMeter,
};

#[test]
fn spherical_entity_mass_is_computed_correctly() {
    let mut store = EntityStore::default();
    let entity = EntityId::new();

    // 1 m radius sphere
    store.add_radius(entity, Radius(Meters(1.0)));

    // Density = 1000 kg/m³ (water)
    store.add_density(
        entity,
        Density(KilogramsPerCubicMeter(1000.0)),
    );

    // Run system
    compute_entity_mass(&mut store);

    // Read derived component
    let mass = store.masses
        .get(&entity)
        .expect("mass should be computed");

    let mass_kg = mass.0 .0;


    // Volume = 4/3 π r³ ≈ 4.18879 m³
    // Mass ≈ 4188.79 kg
    assert!(
        (mass_kg - 4188.79).abs() < 1.0,
        "unexpected mass: {} kg",
        mass_kg
    );
}

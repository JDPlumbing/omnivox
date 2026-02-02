use crate::shared::entities::entity_store::EntityStore;
use crate::core::entity::id::EntityId;
use crate::core::entity::components::geometry_parts::Radius;
use crate::core::entity::systems::geometry::exposure_area::compute_entity_exposure_area;
use crate::core::physics::units::{
    length::Meters,
};

#[test]
fn spherical_entity_exposure_area_is_projected_disk() {
    let mut store = EntityStore::default();
    let entity = EntityId::new();

    // 1 m radius sphere
    store.add_radius(entity, Radius(Meters(1.0)));

    let area = compute_entity_exposure_area(entity, &store)
        .expect("exposure area should exist");

    // Projected area = πr² ≈ 3.14159
    assert!(
        (area.0 - std::f64::consts::PI).abs() < 0.01,
        "unexpected exposure area: {}",
        area.0
    );
}

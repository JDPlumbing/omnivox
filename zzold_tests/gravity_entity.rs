
use crate::core::id::EntityId;
use crate::core::entity::components::world_membership::WorldMembership;
use crate::core::entity::components::position_enu::PositionENU;
use crate::core::entity::components::velocity_enu::VelocityENU;
use crate::core::entity::components::active::Active;
use crate::tests::common::test_simulation_with_earth;
use crate::core::entity::components::location::Location;
use crate::core::physics::units::angle::Radians;
use crate::shared::entities::entity_store::EntityStore;
use crate::core::worlds::id::WorldId;

#[test]
fn entity_falls_downward_under_gravity() {
    let mut store = EntityStore::default();

    let entity = EntityId::new();

    store.world_memberships.insert(
        entity,
        WorldMembership { world_id: WorldId(1) },
    );

    store.locations.insert(
        entity,
        Location {
            latitude: Radians(0.0),
            longitude: Radians(0.0),
        },
    );

    store.position_enus.insert(
        entity,
        PositionENU {
            east: 0.0,
            north: 0.0,
            up: 0.0,
        },
    );

    store.velocity_enus.insert(
        entity,
        VelocityENU {
            ve: 0.0,
            vn: 0.0,
            vu: 0.0,
        },
    );

    store.actives.insert(entity, Active);

    let mut sim = test_simulation_with_earth(store);

    for _ in 0..10 {
        sim.tick();
    }

    let pos = sim.state.entities.position_enus.get(&entity).unwrap();

    assert!(pos.up < 0.0, "entity should have fallen downward");
}

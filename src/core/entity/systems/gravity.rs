use crate::shared::entities::entity_store::EntityStore;
use crate::core::entity::components::acceleration_enu::AccelerationENU;

pub fn apply_gravity_to_entities(
    store: &mut EntityStore,
) {
    for (id, sample) in store.entity_environment_samples.iter() {
        if !store.is_active(id) {
            continue;
        }

        let g = &sample.gravity_enu;

        let applied = AccelerationENU {
            east: g.east,
            north: g.north,
            up: g.up,
        };

        store.acceleration_enus.insert(
            *id,
            applied,
        );
    }
}

use crate::shared::entities::entity_store::EntityStore;
use crate::core::entity::components::weight::Weight;
use crate::core::physics::units::force::Newtons;

pub fn compute_entity_weight(store: &mut EntityStore) {
    for (id, sample) in store.entity_environment_samples.iter() {
        if !store.is_active(id) {
            continue;
        }

        let mass = match store.masses.get(id) {
            Some(m) => m.0 .0,
            None => continue,
        };

        // Weight magnitude = m * g (downward)
        let g = -sample.gravity_enu.up.0;
        let weight = mass * g;

        store.weights.insert(
            *id,
            Weight(Newtons(weight)),
        );
    }
}

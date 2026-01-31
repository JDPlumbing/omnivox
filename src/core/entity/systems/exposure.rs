use crate::shared::entities::entity_store::EntityStore;
use crate::core::physics::units::time::Seconds;
use crate::core::entity::components::exposure::Exposure;

pub fn accumulate_exposure(
    store: &mut EntityStore,
    dt: Seconds,
) {
    for (id, sample) in store.entity_environment_samples.iter() {
        if !store.is_active(id) {
            continue;
        }

        // overwrite, not accumulate
        store.exposures.insert(
            *id,
            Exposure {
                radiant: sample.env.insolation * dt, // J/m² THIS TICK
            },
        );
    }
}

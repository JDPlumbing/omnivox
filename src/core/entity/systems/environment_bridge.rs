use crate::shared::entities::entity_store::EntityStore;
use crate::core::entity::components::entity_environment_sample::EntityEnvironmentSample;
use crate::core::worlds::systems::gravity::LocalENU;
use crate::core::environment::conditions::EnvironmentConditions;
use crate::core::entity::id::EntityId;

/// Attach environment data to an entity.
/// This does NOT compute environment data.
/// It only adapts already-derived facts into entity-local storage.
pub fn declare_entity_environment_sample(
    entity: EntityId,
    env: EnvironmentConditions,
    gravity_enu: LocalENU,
    store: &mut EntityStore,
) {
    store.entity_environment_samples.insert(
        entity,
        EntityEnvironmentSample {
            env,
            gravity_enu,
        },
    );
}

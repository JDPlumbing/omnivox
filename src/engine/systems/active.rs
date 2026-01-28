use crate::shared::entities::entity_store::EntityStore;
use crate::core::{SimTime, EntityId};
use crate::engine::systems::lifecycle::is_entity_active;

pub fn update_active_markers(
    store: &mut EntityStore,
    now: SimTime,
) {
    // First: clear all actives
    store.actives.clear();
    let active_entities: Vec<EntityId> = store
        .spawned_ats
        .keys()
        .copied()
        .filter(|&entity| is_entity_active(store, entity, now))
        .collect();

    // Then: recompute from truth
    for entity in active_entities {
        store.add_active(entity);
    }

}

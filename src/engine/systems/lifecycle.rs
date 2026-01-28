use crate::shared::entities::entity_store::EntityStore;
use crate::core::{EntityId, SimTime};

pub fn is_entity_active(
    store: &EntityStore,
    entity: EntityId,
    now: SimTime,
) -> bool {
    let spawned = match store.spawned_ats.get(&entity) {
        Some(s) => s.time,
        None => return false,
    };

    if spawned > now {
        return false;
    }

    if let Some(despawned) = store.despawned_ats.get(&entity) {
        if despawned.time <= now {
            return false;
        }
    }

    true
}

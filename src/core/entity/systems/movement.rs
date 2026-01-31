use std::collections::HashSet;
use crate::shared::entities::entity_store::EntityStore;
use crate::core::physics::units::time::Seconds;

pub fn integrate_positions(
    store: &mut EntityStore,
    dt: Seconds,
) {
    let active_entities: HashSet<_> =
        store.actives.keys().cloned().collect();

    for (id, pos) in store.position_enus.iter_mut() {
        if !active_entities.contains(id) {
            continue;
        }

        if let Some(vel) = store.velocity_enus.get(id) {
            pos.east  += vel.east  * dt;
            pos.north += vel.north * dt;
            pos.up    += vel.up    * dt;
        }
    }
}

pub fn integrate_velocity(
    store: &mut EntityStore,
    dt: Seconds,
) {
    let active_entities: HashSet<_> =
        store.actives.keys().cloned().collect();

    for (id, vel) in store.velocity_enus.iter_mut() {
        if !active_entities.contains(id) {
            continue;
        }

        if let Some(accel) = store.acceleration_enus.get(id) {
            vel.east  += accel.east  * dt;
            vel.north += accel.north * dt;
            vel.up    += accel.up    * dt;
        }
    }
}

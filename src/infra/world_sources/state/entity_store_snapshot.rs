// infra/world_sources/state/entity_store_snapshot.rs
use serde::{Serialize, Deserialize};
use crate::core::EntityId;
use crate::core::components::{
    spatial::{PositionENU, VelocityENU, world_membership::WorldMembership},
    active::Active,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityStoreSnapshot {
    pub positions: Vec<(EntityId, PositionENU)>,
    pub velocities: Vec<(EntityId, VelocityENU)>,
    pub world_memberships: Vec<(EntityId, WorldMembership)>,
    pub actives: Vec<EntityId>,
}
use crate::shared::entities::entity_store::EntityStore;

impl From<&EntityStore> for EntityStoreSnapshot {
    fn from(store: &EntityStore) -> Self {
        Self {
            positions: store.position_enus
                .iter()
                .map(|(id, v)| (*id, *v))
                .collect(),

            velocities: store.velocity_enus
                .iter()
                .map(|(id, v)| (*id, *v))
                .collect(),

            world_memberships: store.world_memberships
                .iter()
                .map(|(id, v)| (*id, *v))
                .collect(),

            actives: store.actives.keys().cloned().collect(),
        }
    }
}
impl From<EntityStoreSnapshot> for EntityStore {
    fn from(snapshot: EntityStoreSnapshot) -> Self {
        let mut store = EntityStore::default();

        for (id, pos) in snapshot.positions {
            store.position_enus.insert(id, pos);
        }

        for (id, vel) in snapshot.velocities {
            store.velocity_enus.insert(id, vel);
        }

        for (id, wm) in snapshot.world_memberships {
            store.world_memberships.insert(id, wm);
        }

        for id in snapshot.actives {
            store.actives.insert(id, Active);
        }

        store
    }
}

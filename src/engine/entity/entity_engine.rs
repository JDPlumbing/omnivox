use crate::core::{ EntityId, WorldId};
use crate::shared::entities::entity_store::EntityStore;
use crate::engine::entity::errors::DespawnError;
use crate::core::components::{time::time::Time, 
                              meta::note::Note, 
                              spatial::world_membership::WorldMembership, 
                              spatial::position::Position,
                              time::spawned_at::SpawnedAt,
                              time::despawned_at::DespawnedAt,

                            };




pub struct EntityEngine<'a> {
    store: &'a mut EntityStore,
}

impl<'a> EntityEngine<'a> {
    pub fn new(store: &'a mut EntityStore) -> Self {
        Self { store }
    }

    pub fn create_time_marker(&mut self, time: Time) -> EntityId {
        let entity = EntityId::new();
        self.store.add_time(entity, time);
        entity
    }

    pub fn add_note(&mut self, entity: EntityId, note: Note) {
        self.store.add_note(entity, note);
    }

    pub fn create_note_entity(&mut self, note: Note) -> EntityId {
        let entity = EntityId::new();
        self.store.add_note(entity, note);
        entity
    }

    pub fn set_world(
        &mut self,
        entity: EntityId,
        world_id: WorldId,
    ) {
        self.store.add_world_membership(
            entity,
            WorldMembership { world_id },
        );
    }

    pub fn set_position(
        &mut self,
        entity: EntityId,
        position: Position,
    ) {
        self.store.add_position(
            entity,
            position,
        );
    }
    pub fn set_spawned_at(
        &mut self,
        entity: EntityId,
        spawned_at: SpawnedAt,
    ) {
        self.store.add_spawned_at(
            entity,
            spawned_at,
        );
    }
    pub fn set_despawned_at(
        &mut self,
        entity: EntityId,
        despawned_at: DespawnedAt,
    ) -> Result<(), DespawnError> {
        let spawned = self.store.spawned_ats.get(&entity)
            .ok_or(DespawnError::NotSpawned)?;

        if despawned_at.time <= spawned.time {
            return Err(DespawnError::InvalidTime);
        }

        self.store.add_despawned_at(entity, despawned_at);
        Ok(())
    }


}
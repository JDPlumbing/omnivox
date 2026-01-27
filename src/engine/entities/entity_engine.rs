use crate::core::{ EntityId, WorldId};
use crate::shared::entities::entity_store::EntityStore;
use crate::core::components::{time::Time, 
                              note::Note, 
                              world_membership::WorldMembership, 
                              position::Position,

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

}
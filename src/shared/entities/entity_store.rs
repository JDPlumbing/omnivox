// shared/entities/entity_store.rs
use std::collections::HashMap;
use crate::core::EntityId;
use crate::core::components::time::Time;
use crate::core::components::note::Note;
use crate::core::components::world_membership::WorldMembership;
use crate::core::components::position::Position;

#[derive(Default)]
pub struct EntityStore {
    pub times: HashMap<EntityId, Time>,
    pub notes: HashMap<EntityId, Note>,
    pub world_memberships: HashMap<EntityId, WorldMembership>,
    pub positions: HashMap<EntityId, Position>,
}

impl EntityStore {
    pub fn new() -> Self {
        Self {
            times: HashMap::new(),
            notes: HashMap::new(),
            world_memberships: HashMap::new(),
            positions: HashMap::new(),
        }
    }

    pub fn add_time(&mut self, entity: EntityId, time: Time) {
        self.times.insert(entity, time);
    }

    pub fn add_note(&mut self, entity: EntityId, note: Note ) {
        self.notes.insert(entity, note);
    }

    pub fn add_world_membership(&mut self, entity: EntityId, membership: WorldMembership) {
        self.world_memberships.insert(entity, membership);
    }

    pub fn add_position(&mut self, entity: EntityId, position: Position) {
        self.positions.insert(entity, position);
    }
}

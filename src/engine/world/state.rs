use std::collections::{HashMap, HashSet};
use serde_json::Value;

// Identity
use crate::core::id::{EntityId, WorldId};
//World metadata (NOT runtime logic)
use crate::core::world::{World, WorldEnvironment};
// Time ( value types )
use crate::core::tdt::sim_clock::SimClock;
use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_duration::SimDuration;

// Components
use crate::core::components::orientation::Orientation;
use crate::core::components::position::Position;
use crate::core::components::lifecycle::Lifecycle;
use crate::core::components::{ShapeRef, MaterialRef};




/// -------------------------------------------------------------------
/// ECS-style in-memory simulation state for a running world
/// -------------------------------------------------------------------

pub struct WorldState {
    // --- World ---
    pub meta: World,
    pub environment: WorldEnvironment,

    // --- Entity registry ---
    pub entities: HashSet<EntityId>,

    // --- Components ---
    pub world_membership: HashMap<EntityId, WorldId>,
    pub positions: HashMap<EntityId, Position>,
    pub orientations: HashMap<EntityId, Orientation>,
    pub lifecycles: HashMap<EntityId, Lifecycle>,
    pub metadata: HashMap<EntityId, Value>,

    // --- Time ---
    pub sim_time: SimTime,
    pub sim_delta: SimDuration,
    pub clock: Option<SimClock>,
}


impl WorldState {
    pub fn new(meta: World, environment: WorldEnvironment) -> Self {
        Self {
            meta,
            environment,

            entities: HashSet::new(),

            world_membership: HashMap::new(),
            positions: HashMap::new(),
            orientations: HashMap::new(),
            //shapes: HashMap::new(),
            //materials: HashMap::new(),
            lifecycles: HashMap::new(),
            metadata: HashMap::new(),

            sim_time: SimTime::from_ns(0),
            sim_delta: SimDuration::from_ns(0),
            clock: None,
        }
    }

    /// Spawn a new entity and attach core components
    pub fn spawn_entity(&mut self) -> EntityId {
        let id = EntityId::new();
        self.entities.insert(id);
        id
    }

    /// Despawn an entity (logical removal)
    pub fn despawn_entity(&mut self, id: EntityId, t: SimTime) {
        if let Some(lifecycle) = self.lifecycles.get_mut(&id) {
            lifecycle.despawned_at = Some(t);
        }
    }

    /// Hard delete (use sparingly)
    pub fn delete_entity(&mut self, id: EntityId) {
        self.entities.remove(&id);
        self.remove_all_components(id);
    }

    /// Remove all components for an entity
    fn remove_all_components(&mut self, id: EntityId) {
        self.world_membership.remove(&id);
        self.positions.remove(&id);
        self.orientations.remove(&id);
        self.lifecycles.remove(&id);
        self.metadata.remove(&id);
    }
}
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_duration::SimDuration;
use crate::core::id::WorldId;

use crate::engine::world::state::{WorldState, World};
use crate::engine::entities::SimEntity;
use crate::engine::components::SimComponents;

use crate::supabasic::worlds::WorldRow;
use crate::engine::simulations::simulation::Simulation;
use crate::engine::simulations::simulation_config::SimulationConfig;
use crate::core::id::EntityId;

/// ===============================================================
/// Persisted entity
/// ===============================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedEntity {
    pub id: EntityId,
    pub json: serde_json::Value,
}

impl PersistedEntity {
    pub fn from_entity(ent: &SimEntity) -> Self {
        Self {
            id: ent.id, // direct copy, EntityId is Copy + Serialize
            json: serde_json::to_value(ent).unwrap(),
        }
    }

    pub fn to_entity(&self) -> SimEntity {
        serde_json::from_value(self.json.clone()).unwrap()
    }
}


/// ===============================================================
/// Persisted components
/// ===============================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedComponents {
    pub json: Value,
}

impl PersistedComponents {
    pub fn from_world(world: &WorldState) -> Self {
        Self {
            json: serde_json::to_value(&world.components).unwrap(),
        }
    }

    pub fn apply_to_world(&self, world: &mut WorldState) {
        world.components = serde_json::from_value(self.json.clone()).unwrap();
    }
}

/// ===============================================================
/// Persisted FULL simulation state
/// ===============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedSimState {
    pub world_id: WorldId,

    pub sim_time: SimTime,
    pub sim_delta: SimDuration,

    pub world_meta: serde_json::Value,
    pub entities: Vec<PersistedEntity>,
    pub components: PersistedComponents,

    pub metadata: serde_json::Value,
}

impl PersistedSimState {
    pub fn from_runtime(sim: &Simulation) -> Self {
        let world = &sim.world;

        let entities = world.entities
            .values()
            .map(PersistedEntity::from_entity)
            .collect();

        Self {
            world_id: world.meta.id,
            sim_time: world.sim_time,
            sim_delta: world.sim_delta,
            world_meta: serde_json::to_value(&world.meta).unwrap(),
            entities,
            components: PersistedComponents::from_world(world),
            metadata: serde_json::json!({}),
        }
    }

    pub fn to_runtime(
        self,
        world_record: WorldRow,
        cfg: SimulationConfig,
    ) -> Simulation {

        let meta_world: World = serde_json::from_value(self.world_meta)
            .expect("world_meta decode failed");

        let mut world_state = WorldState::new(meta_world);

        world_state.sim_time = self.sim_time;
        world_state.sim_delta = self.sim_delta;

        for ent in &self.entities {
            let entity = ent.to_entity();
            world_state.entities.insert(entity.id, entity);
        }

        self.components.apply_to_world(&mut world_state);

        let mut sim = Simulation::new_from_config(&cfg, world_record);
        sim.world = world_state;
        sim
    }
}
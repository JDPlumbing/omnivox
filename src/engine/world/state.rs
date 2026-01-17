
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::collections::HashMap;

use crate::supabasic::events::EventRow;
use crate::supabasic::worlds::WorldRow;

use crate::core::tdt::sim_clock::SimClock;
//use crate::engine::components::SimComponents;
use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_duration::SimDuration;

use crate::core::SimEntity;
use crate::core::id::{WorldId, EntityId};

use crate::core::world::World;
use crate::core::world::WorldEnvironment;



/// -------------------------------------------------------------------
/// In-memory simulation state for a running world
/// -------------------------------------------------------------------

pub struct WorldState {
    pub meta: World,
    pub environment: WorldEnvironment,
    pub entities: HashMap<EntityId, SimEntity>,
    pub free_list: Vec<u32>,
    pub generations: Vec<u32>,
    pub sim_time: SimTime,
    pub sim_delta: SimDuration,
    pub clock: Option<SimClock>,
    //pub components: SimComponents,
}
impl WorldState {
    pub fn allocate_entity_id(&mut self) -> EntityId {
        if let Some(index) = self.free_list.pop() {
            let r#gen = self.generations[index as usize];
            EntityId::new(index, r#gen)
        } else {
            let index = self.generations.len() as u32;
            self.generations.push(0);
            EntityId::new(index, 0)
        }
    }

    pub fn free_entity_id(&mut self, id: EntityId) {
        self.generations[id.index as usize] += 1;
        self.free_list.push(id.index);
    }

    pub fn new(meta: World, environment: WorldEnvironment) -> Self {
        Self {
            meta,
            environment,
            entities: HashMap::new(),
            free_list: Vec::new(),
            generations: Vec::new(),
            sim_time: SimTime::from_ns(0),
            sim_delta: SimDuration::from_ns(0),
            clock: None,
            //components: SimComponents::new(),
        }
    }

    pub fn from_entities(meta: World, environment: WorldEnvironment, entities: Vec<SimEntity>) -> Self {
        let mut map = HashMap::new();
        for ent in entities {
            map.insert(ent.id, ent);
        }

        Self {
            meta,
            environment,
            entities: map,
            free_list: Vec::new(),
            generations: Vec::new(),
            //events: Vec::new(),
            sim_time: SimTime::from_ns(0),
            sim_delta: SimDuration::from_ns(0),
            clock: None,
            //components: SimComponents::new(),
        }
    }
}


impl From<WorldRow> for World {
    fn from(rec: WorldRow) -> Self {
        World {
            id: rec.world_id,
            name: rec.name,
            description: rec.description,
            world_epoch: rec.world_epoch
                .as_ref()
                .and_then(|s| s.parse::<i128>().ok())
                .map(SimTime::from_ns),

        }
    }
}


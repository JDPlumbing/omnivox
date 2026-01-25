use anyhow::{Result, anyhow};
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::core::id::{WorldId, EntityId};
use crate::core::tdt::sim_time::SimTime;
use crate::core::UvoxId;

use crate::core::components::{
    position::Position,
    orientation::Orientation,
    lifecycle::Lifecycle,
};

use crate::core::world::{World, WorldEnvironment};
use crate::engine::world::state::WorldState;

use crate::supabasic::entity::EntityRow;
use crate::supabasic::worlds::WorldRow;

use super::source::WorldSource;
use super::json_world::JsonWorldFile;
use crate::core::world::presets::earth_v0;
use serde_json;
use serde::{Deserialize, Serialize};
use crate::supabasic::worlds::NewWorldRow;
use crate::core::world::WorldStats;

pub struct JsonWorldSource {
    worlds: HashMap<WorldId, JsonWorldFile>,
}

impl JsonWorldSource {
    pub fn from_dir(path: impl AsRef<Path>) -> Result<Self> {
        let mut worlds = HashMap::new();

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let text = std::fs::read_to_string(entry.path())?;
            let file: JsonWorldFile = serde_json::from_str(&text)?;

            worlds.insert(file.world.world_id, file);
        }

        Ok(Self { worlds })
    }

    fn hydrate_world(file: &JsonWorldFile) -> Result<WorldState> {
        let env_desc = file
            .world
            .environment
            .clone()
            .unwrap_or_else(earth_v0);

        let world_env = WorldEnvironment::from_descriptor(&env_desc);

        let meta = World {
            id: file.world.world_id,
            name: file.world.name.clone(),
            description: file.world.description.clone(),
            world_epoch: file.world.world_epoch
                .as_ref()
                .and_then(|s| s.parse::<i128>().ok())
                .map(SimTime::from_ns),
        };

        let mut state = WorldState::new(meta, world_env);

        for row in &file.entities {
            let id = EntityId(row.row_id.ok_or_else(|| {
                anyhow!("EntityRow missing row_id")
            })?);

            state.entities.insert(id);
            state.world_membership.insert(id, file.world.world_id);

            let position: UvoxId =
                serde_json::from_value(row.position.clone())?;
            state.positions.insert(id, Position(position));

            if !row.orientation.is_null() {
                let o = serde_json::from_value(row.orientation.clone())?;
                state.orientations.insert(id, Orientation(o));
            }

            state.lifecycles.insert(id, Lifecycle {
                spawned_at: row.spawned_at,
                despawned_at: row.despawned_at,
            });

            state.metadata.insert(id, row.metadata.clone());
        }

        Ok(state)
    }
}

#[async_trait]
impl WorldSource for JsonWorldSource {
    async fn list_worlds(&self) -> Result<Vec<WorldRow>> {
        Ok(self.worlds.values().map(|f| f.world.clone()).collect())
    }

    async fn load_world(&self, world_id: WorldId) -> Result<WorldState> {
        let file = self
            .worlds
            .get(&world_id)
            .ok_or_else(|| anyhow!("World not found in JSON source"))?;

        Self::hydrate_world(file)
    }
    async fn get_world(&self, world_id: WorldId) -> Result<WorldRow> {
        let file = self
            .worlds
            .get(&world_id)
            .ok_or_else(|| anyhow!("World not found in JSON source"))?;

        Ok(file.world.clone())
    }
    async fn create_world(&self, _payload: NewWorldRow) -> Result<WorldRow> {
        Err(anyhow!("JSON WorldSource is read-only"))
    }
    async fn update_world(&self, _world_id: WorldId, _changes: serde_json::Value) -> Result<WorldRow> {
        Err(anyhow!("JSON WorldSource is read-only"))
    }
    async fn delete_world(&self, _world_id: WorldId) -> Result<()> {
        Err(anyhow!("JSON WorldSource is read-only"))
    }
    async fn world_stats(&self, _world_id: WorldId) -> Result<WorldStats> {
        Err(anyhow!("JSON WorldSource is read-only"))
    }
    async fn set_world_epoch(
        &self,
        _world_id: WorldId,
        _epoch: SimTime,
    ) -> Result<()> {
        Err(anyhow!("JSON WorldSource is read-only"))
    }
}

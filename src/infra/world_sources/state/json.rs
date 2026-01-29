use anyhow::Result;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use std::fs;

use crate::core::id::WorldId;
use crate::core::tdt::sim_time::SimTime;

use crate::infra::world_sources::state::entity_store_snapshot::EntityStoreSnapshot;
use crate::shared::world_sources::state::source::{
    WorldStateSource,
    WorldStateSnapshot,
};

use serde::{Serialize, Deserialize};

/// What actually gets written to disk
#[derive(Debug, Serialize, Deserialize)]
struct WorldStateFile {
    sim_time: i128,
    entity_store: EntityStoreSnapshot,
}

pub struct JsonWorldStateSource {
    root: PathBuf,
}

impl JsonWorldStateSource {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    fn state_path(&self, world_id: WorldId) -> PathBuf {
        self.root.join(format!("world_{}.json", world_id))
    }
}

#[async_trait]
impl WorldStateSource for JsonWorldStateSource {
    async fn load_snapshot(
        &self,
        world_id: WorldId,
    ) -> Result<Option<WorldStateSnapshot>> {
        let path = self.state_path(world_id);

        if !path.exists() {
            return Ok(None);
        }

        let raw = fs::read_to_string(&path)?;
        let file: WorldStateFile = serde_json::from_str(&raw)?;

        Ok(Some(WorldStateSnapshot {
            sim_time: SimTime::from_ns(file.sim_time),
            entity_store_snapshot: file.entity_store,
        }))
    }

    async fn save_snapshot(
        &self,
        world_id: WorldId,
        snapshot: &WorldStateSnapshot,
    ) -> Result<()> {
        let path = self.state_path(world_id);

        let file = WorldStateFile {
            sim_time: snapshot.sim_time.as_ns(),
            entity_store: snapshot.entity_store_snapshot.clone(),
        };

        let json = serde_json::to_string_pretty(&file)?;
        fs::write(path, json)?;

        Ok(())
    }
}

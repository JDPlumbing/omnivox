use std::fs;
use std::path::PathBuf;

use anyhow::{Result, bail};
use async_trait::async_trait;

use crate::core::id::WorldId;
use crate::core::tdt::sim_time::SimTime;
use crate::core::world::catalog::{ NewWorld, WorldUpdate};
use crate::core::world::WorldStats;
use crate::core::world::world_definition::WorldDefinition;
use crate::core::world::world_summary::WorldSummary;
use crate::shared::world_sources::catalog::source::WorldCatalog;


/// JSON-backed, read-only world catalog
pub struct JsonWorldCatalog {
    root: PathBuf,
    worlds: Vec<WorldDefinition>,
}

impl JsonWorldCatalog {
    pub fn from_dir<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let root = path.into();
        let mut worlds = Vec::new();

        for entry in fs::read_dir(&root)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                let data = fs::read_to_string(&path)?;
                let mut defs: Vec<WorldDefinition> = serde_json::from_str(&data)?;
                worlds.append(&mut defs);
            }
        }

        Ok(Self { root, worlds })
    }

    //fn find(&self, world_id: WorldId) -> Option<&WorldDefinition> {
    //    self.worlds.iter().find(|w| w.world_id == world_id)
    //}

    fn definition_path(&self, world_id: WorldId) -> std::path::PathBuf {
        self.root.join(format!("world_{}.json", world_id))
    }
}

#[async_trait]
impl WorldCatalog for JsonWorldCatalog {
    async fn list_worlds(&self) -> Result<Vec<WorldSummary>> {
        Ok(self.worlds.iter().map(WorldSummary::from).collect())
    }
        async fn get_world_definition(
            &self,
            world_id: WorldId,
        ) -> Result<WorldDefinition> {
            let path = self.definition_path(world_id);
            let raw = std::fs::read_to_string(path)?;
            let def: WorldDefinition = serde_json::from_str(&raw)?;
            Ok(def)
        }

    async fn get_world(&self, world_id: WorldId) -> Result<WorldSummary> {
        let def = self
            .worlds
            .iter()
            .find(|w| w.world_id == world_id)
            .ok_or_else(|| anyhow::anyhow!("World not found"))?;

        Ok(WorldSummary::from(def))
    }


    async fn create_world(&self, _: NewWorld) -> Result<WorldSummary> {
        bail!("JSON world catalog is read-only")
    }

    async fn update_world(
        &self,
        _: WorldId,
        _: WorldUpdate,
    ) -> Result<WorldSummary> {
        bail!("JSON world catalog is read-only")
    }

    async fn delete_world(&self, _: WorldId) -> Result<()> {
        bail!("JSON world catalog is read-only")
    }

    async fn world_stats(&self, _: WorldId) -> Result<WorldStats> {
        bail!("World stats not supported for JSON worlds")
    }

    async fn set_world_epoch(
        &self,
        _: WorldId,
        _: SimTime,
    ) -> Result<()> {
        bail!("World epoch cannot be updated for JSON worlds")
    }
}

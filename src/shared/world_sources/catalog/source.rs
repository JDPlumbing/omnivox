use anyhow::Result;
use async_trait::async_trait;

use crate::core::id::WorldId;
use crate::core::world::WorldStats;
use crate::core::tdt::sim_time::SimTime;
use crate::core::world::catalog::{ NewWorld, WorldUpdate};
use crate::core::world::world_summary::WorldSummary;
use crate::core::world::WorldDefinition;

#[async_trait]
pub trait WorldCatalog: Send + Sync {
    // ---- UI / listing ----
    async fn list_worlds(&self) -> Result<Vec<WorldSummary>>;
    async fn get_world(&self, world_id: WorldId) -> Result<WorldSummary>;

    // ---- simulation / loader ----
    async fn get_world_definition(
        &self,
        world_id: WorldId,
    ) -> Result<WorldDefinition>;

    // ---- mutation ----
    async fn create_world(&self, payload: NewWorld) -> Result<WorldSummary>;
    async fn update_world(
        &self,
        world_id: WorldId,
        changes: WorldUpdate,
    ) -> Result<WorldSummary>;
    async fn delete_world(&self, world_id: WorldId) -> Result<()>;

    // ---- stats / ops ----
    async fn world_stats(&self, world_id: WorldId) -> Result<WorldStats>;
    async fn set_world_epoch(
        &self,
        world_id: WorldId,
        epoch: SimTime,
    ) -> Result<()>;
}

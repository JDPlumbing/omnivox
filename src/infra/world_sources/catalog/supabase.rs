use anyhow::{Result, anyhow};
use async_trait::async_trait;

use crate::core::id::WorldId;
use crate::core::tdt::sim_time::SimTime;
use crate::core::world::catalog::{WorldSummary, NewWorld, WorldUpdate};
use crate::core::world::WorldStats;

use crate::shared::world_sources::catalog::source::WorldCatalog;
use crate::supabasic::Supabase;
use crate::supabasic::worlds::{WorldRow, NewWorldRow};


/// Supabase-backed world catalog
pub struct SupabaseWorldCatalog {
    supa: Supabase,
}

impl SupabaseWorldCatalog {
    pub fn new(supa: Supabase) -> Self {
        Self { supa }
    }
}

#[async_trait]
impl WorldCatalog for SupabaseWorldCatalog {
    async fn list_worlds(&self) -> Result<Vec<WorldSummary>> {
        let rows = WorldRow::list(&self.supa).await?;

        rows.into_iter()
            .map(WorldSummary::try_from)
            .collect()
    }

    async fn get_world(&self, world_id: WorldId) -> Result<WorldSummary> {
        let row = WorldRow::get(&self.supa, world_id).await?;
        WorldSummary::try_from(row)
    }

    async fn create_world(&self, payload: NewWorld) -> Result<WorldSummary> {
        let row = NewWorldRow::from(payload)
            .insert(&self.supa)
            .await?;

        WorldSummary::try_from(row)
    }

    async fn update_world(
        &self,
        world_id: WorldId,
        changes: WorldUpdate,
    ) -> Result<WorldSummary> {
        let row = WorldRow::update(
            &self.supa,
            world_id,
            changes.into(),
        )
        .await?;

        WorldSummary::try_from(row)
    }

    async fn delete_world(&self, world_id: WorldId) -> Result<()> {
        WorldRow::delete(&self.supa, world_id).await?;
        Ok(())
    }

    async fn world_stats(&self, world_id: WorldId) -> Result<WorldStats> {
        WorldRow::stats(&self.supa, world_id).await?
            .ok_or_else(|| anyhow!("World stats unavailable"))
    }

    async fn set_world_epoch(
        &self,
        world_id: WorldId,
        epoch: SimTime,
    ) -> Result<()> {
        WorldRow::set_world_epoch(
            &self.supa,
            world_id,
            epoch.as_ns(),
        )
        .await?;

        Ok(())
    }
}

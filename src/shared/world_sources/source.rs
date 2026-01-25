use anyhow::Result;
use async_trait::async_trait;

use crate::core::id::WorldId;
use crate::engine::world::state::WorldState;
use crate::supabasic::worlds::WorldRow;
use crate::core::world::WorldStats;
use crate::supabasic::worlds::NewWorldRow;
use crate::core::tdt::sim_time::SimTime;
use serde_json;

#[async_trait]
pub trait WorldSource: Send + Sync {
    async fn list_worlds(&self) -> Result<Vec<WorldRow>>;
    async fn get_world(&self, world_id: WorldId) -> Result<WorldRow>;
    async fn create_world(&self, payload: NewWorldRow) -> Result<WorldRow>;
    async fn update_world(&self, world_id: WorldId, changes: serde_json::Value) -> Result<WorldRow>;
    async fn delete_world(&self, world_id: WorldId) -> Result<()>;
    async fn world_stats(&self, world_id: WorldId) -> Result<WorldStats>;
    async fn set_world_epoch(&self,world_id: WorldId,epoch: SimTime,) -> Result<()>;

    async fn load_world(&self, world_id: WorldId) -> Result<WorldState>;
}


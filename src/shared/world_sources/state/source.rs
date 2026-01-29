use async_trait::async_trait;
use anyhow::Result;

use crate::core::id::WorldId;

use crate::infra::world_sources::state::entity_store_snapshot::EntityStoreSnapshot;
use crate::core::tdt::sim_time::SimTime;

#[derive(Clone)]
pub struct WorldStateSnapshot {
    pub sim_time: SimTime,
    pub entity_store_snapshot: EntityStoreSnapshot,
}

#[async_trait]
pub trait WorldStateSource: Send + Sync {
    async fn load_snapshot(
        &self,
        world_id: WorldId,
    ) -> Result<Option<WorldStateSnapshot>>;

    async fn save_snapshot(
        &self,
        world_id: WorldId,
        snapshot: &WorldStateSnapshot,
    ) -> Result<()>;
}

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;

use crate::core::id::WorldId;
use crate::core::UserId;
use crate::engine::world::state::WorldState;
use crate::shared::world_sources::source::WorldSource;

pub struct WorldEngine {
    /// Persistent world loader (infra-backed)
    pub world_source: Arc<dyn WorldSource + Send + Sync>,

    /// In-memory active worlds (runtime authority)
    pub worlds: Arc<RwLock<HashMap<WorldId, Arc<RwLock<WorldState>>>>>,
}

impl WorldEngine {
    /// Enter (or load) a world runtime.
    ///
    /// This is the **only** place where a WorldState becomes active.
    pub async fn enter_world(
        &self,
        _actor: UserId, // permissions later
        world_id: WorldId,
    ) -> Result<Arc<RwLock<WorldState>>> {
        // 1️⃣ Fast path: already active
        if let Some(existing) = self.worlds.read().await.get(&world_id) {
            return Ok(existing.clone());
        }

        // 2️⃣ Load runtime state from persistence
        let world_state = self.world_source.load_world(world_id).await?;

        let world_state = Arc::new(RwLock::new(world_state));

        // 3️⃣ Register as active
        self.worlds
            .write()
            .await
            .insert(world_id, world_state.clone());

        Ok(world_state)
    }
}

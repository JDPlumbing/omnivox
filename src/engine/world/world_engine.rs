use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;

use crate::core::id::WorldId;
use crate::engine::world::state::WorldState;
use crate::engine::world::loader::WorldLoader;

pub struct WorldEngine {
    pub loader: Arc<WorldLoader>,
    pub worlds: Arc<RwLock<HashMap<WorldId, Arc<RwLock<WorldState>>>>>,
}

impl WorldEngine {
    pub fn new(loader: Arc<WorldLoader>) -> Self {
        Self {
            loader,
            worlds: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn load_world(
        &self,
        world_id: WorldId,
    ) -> Result<Arc<RwLock<WorldState>>> {
        // 1️⃣ Check runtime cache
        {
            let worlds = self.worlds.read().await;
            if let Some(existing) = worlds.get(&world_id) {
                return Ok(existing.clone());
            }
        }

        // 2️⃣ Load via loader (authoritative path)
        let state = self.loader.load(world_id).await?;
        let state = Arc::new(RwLock::new(state));

        // 3️⃣ Cache
        self.worlds
            .write()
            .await
            .insert(world_id, state.clone());

        Ok(state)
    }
}

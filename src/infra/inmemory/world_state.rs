use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::Result;
use async_trait::async_trait;

use crate::core::id::WorldId;
use crate::shared::world_sources::state::source::{
    WorldStateSource,
    WorldStateSnapshot,
};

/// --------------------------------------------------
/// In-memory WorldStateSource
/// --------------------------------------------------
///
/// - Stores runtime ECS snapshots in memory
/// - No persistence
/// - Used for dev / tests / hot worlds
/// --------------------------------------------------

pub struct InMemoryWorldStateSource {
    snapshots: Mutex<HashMap<WorldId, WorldStateSnapshot>>,
}

impl Default for InMemoryWorldStateSource {
    fn default() -> Self {
        Self {
            snapshots: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl WorldStateSource for InMemoryWorldStateSource {
    async fn load_snapshot(
        &self,
        world_id: WorldId,
    ) -> Result<Option<WorldStateSnapshot>> {
        let map = self.snapshots.lock().unwrap();
        Ok(map.get(&world_id).cloned())
    }

    async fn save_snapshot(
        &self,
        world_id: WorldId,
        snapshot: &WorldStateSnapshot,
    ) -> Result<()> {
        self.snapshots
            .lock()
            .unwrap()
            .insert(world_id, snapshot.clone());

        Ok(())
    }
}

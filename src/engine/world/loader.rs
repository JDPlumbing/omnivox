use anyhow::{Result, anyhow};
use std::sync::Arc;
use crate::core::world::World; 
use crate::core::id::WorldId;
use crate::shared::entities::entity_store::EntityStore;
use crate::core::tdt::sim_time::SimTime;
use crate::engine::world::state::WorldState;
use crate::core::world::WorldEnvironment;
use crate::shared::world_sources::catalog::source::WorldCatalog;
use crate::shared::world_sources::state::source::{
    WorldStateSource, WorldStateSnapshot,
};

pub struct WorldLoader {
    catalog: Arc<dyn WorldCatalog + Send + Sync>,
    state_source: Arc<dyn WorldStateSource + Send + Sync>,
}

impl WorldLoader {
    pub fn new(
        catalog: Arc<dyn WorldCatalog + Send + Sync>,
        state_source: Arc<dyn WorldStateSource + Send + Sync>,
    ) -> Self {
        Self { catalog, state_source }
    }

    pub async fn load(&self, world_id: WorldId) -> Result<WorldState> {
        // 1️⃣ Load full definition
        let def = self.catalog.get_world_definition(world_id).await?;

        // 2️⃣ Build environment (this is why we needed definition)
        let env_desc = def.environment
            .as_ref()
            .ok_or_else(|| anyhow!("World has no environment descriptor"))?;

        let environment = WorldEnvironment::from_descriptor(env_desc);

        // 3️⃣ Build World meta
        let world = World::new(
            def.world_id,
            Some(def.name),
            def.description,
            None, // or parse epoch later
        );

        // 4️⃣ Load snapshot
        let snapshot = self
            .state_source
            .load_snapshot(world_id)
            .await?
            .unwrap_or_else(|| WorldStateSnapshot {
                sim_time: SimTime::from_ns(0),
                entity_store: EntityStore::default(),
            });

        // 5️⃣ Construct authoritative runtime state
        Ok(WorldState::from_snapshot(
            world,
            environment,
            snapshot,
        ))
    }

}

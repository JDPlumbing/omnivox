use crate::core::world::{World, WorldEnvironment};
use crate::core::tdt::sim_time::SimTime;
use crate::shared::entities::entity_store::EntityStore;

/// -------------------------------------------------------------------
/// ECS-style in-memory simulation state for a running world
/// -------------------------------------------------------------------
#[derive(Clone)]
pub struct WorldState {
    // --- World metadata (static / descriptive) ---
    pub meta: World,
    pub environment: WorldEnvironment,
    // --- Runtime ---
    pub sim_time: SimTime,
    // --- ECS ---
    pub entity_store: EntityStore,

    // --- Derived / runtime-only helpers (later) ---
    // pub spatial_index: SpatialIndex,
    // pub system_cache: ...
}

impl WorldState {
    pub fn new(meta: World, environment: WorldEnvironment) -> Self {
        Self {
            meta,
            environment,
            sim_time: SimTime::from_ns(0),
            entity_store: EntityStore::default(),
        }
    }
}

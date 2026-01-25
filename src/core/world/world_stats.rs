// core/world/world_stats.rs
use crate::core::id::WorldId;

#[derive(Debug, Clone)]
pub struct WorldStats {
    pub world_id: WorldId,
    pub entity_count: u64,
}

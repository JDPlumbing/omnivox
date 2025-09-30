use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Data Transfer Object for SimWorld
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimWorldDto {
    pub simulation_id: Uuid,
    pub frame_id: u64,
    pub tick_rate_ns: i64,
    pub current_tick: i64,
    pub persist_events: bool,
}

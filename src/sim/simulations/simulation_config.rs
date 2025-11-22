use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::core::tdt::sim_duration::SimDuration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub simulation_id: Uuid,
    pub world_id: i64,

    /// Simulation step size (e.g. 60 simulated seconds per tick)
    pub dt: SimDuration,

    /// Optional speed multiplier (1 = realtime, 100 = fast-forward)
    pub speed: f64,

    pub metadata: serde_json::Value,
}

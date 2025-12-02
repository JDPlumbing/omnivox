use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::supabasic::simulations::SimulationRow;
use crate::supabasic::events::EventRow;
use crate::core::id::{SimulationId, WorldId};

/// DTO used for simulation API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationDto {
    pub simulation_id: SimulationId,
    pub user_owner_id: Option<Uuid>,
    pub anon_owner_id: Option<Uuid>,
    pub tick_rate: i64,
    pub world_id: WorldId,
    pub last_saved: Option<String>,
    #[serde(default)]
    pub events: Vec<EventRow>,
}

/// Payload used when creating new simulations
#[derive(Debug, Serialize, Deserialize)]
pub struct NewSimulation {
    pub simulation_id: Option<SimulationId>, // optional; generated if missing
    pub world_id: WorldId,
    pub tick_rate: i64,
    pub last_saved: Option<chrono::DateTime<chrono::Utc>>,
    pub metadata: Option<serde_json::Value>,
    pub user_owner_id: Option<Uuid>,
    pub anon_owner_id: Option<Uuid>,
}

/// Conversion from DB row → DTO for API output
impl From<SimulationRow> for SimulationDto {
    fn from(row: SimulationRow) -> Self {
        SimulationDto {
            simulation_id: row.simulation_id,
            user_owner_id: row.user_owner_id,
            anon_owner_id: row.anon_owner_id,
            tick_rate: row.tick_rate,
            world_id: row.world_id,
            last_saved: row.last_saved.map(|dt| dt.to_rfc3339()),
            events: vec![],
        }
    }
}

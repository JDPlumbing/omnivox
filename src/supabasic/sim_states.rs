use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::supabasic::{Supabase, SupabasicError};
use crate::core::id::SimulationId;
use crate::sim::simulations::persist::state::PersistedSimState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimStateRow {
    pub id: Option<i64>,
    pub simulation_id: SimulationId,
    pub created_at: DateTime<Utc>,
    pub snapshot: serde_json::Value,
}

impl SimStateRow {
    pub async fn insert(
        supa: &Supabase,
        sim_id: SimulationId,
        snapshot: &PersistedSimState
    ) -> Result<(), SupabasicError> {

        let json = serde_json::to_value(snapshot)?;
        
        supa.from("sim_states")
            .insert(serde_json::json!([{
                "simulation_id": sim_id.to_string(),
                "snapshot": json
            }]))
            .execute()
            .await?;

        Ok(())
    }

    pub async fn latest(
        supa: &Supabase,
        sim_id: SimulationId,
    ) -> Result<SimStateRow, SupabasicError> {

        let rows: Vec<SimStateRow> = supa
            .from("sim_states")
            .select("*")
            .eq("simulation_id", &sim_id.to_string())
            .order("created_at.desc")
            .limit(1)
            .execute_typed()
            .await?;

        rows.into_iter().next().ok_or_else(|| {
            SupabasicError::Other("No snapshot found".into())
        })
    }
}

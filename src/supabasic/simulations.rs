use crate::supabasic::client::Supabase;
use crate::supabasic::{SupabasicError};
use crate::supabasic::orm::DbModel;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::core::id::{SimulationId, WorldId};

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationRow {
    pub simulation_id: SimulationId,           // JSONB <—
    pub user_owner_id: Option<uuid::Uuid>,
    pub anon_owner_id: Option<uuid::Uuid>,
    pub tick_rate: i64,
    pub world_id: WorldId,
    pub last_saved: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,   // Alignment field (good)
}

#[derive(Debug, Deserialize)]
pub struct UpdateSimulation {
    pub world_id: Option<WorldId>,
    pub tick_rate: Option<i64>,
    pub anon_owner_id: Option<uuid::Uuid>,
    pub user_owner_id: Option<uuid::Uuid>,
    pub last_saved: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

impl DbModel for SimulationRow {
    fn table() -> &'static str { "simulations" }
}

impl SimulationRow {

    pub async fn get(
        supa: &Supabase,
        sim_id: SimulationId
    ) -> Result<Self, SupabasicError> {

        let json_id = serde_json::to_value(&sim_id)
            .expect("SimulationId JSON encode");

        let val = supa
            .from(Self::table())
            .select("*")
            //.eq("simulation_id", serde_json::json!(sim_id)) // 🔥 JSONB compare
            .single()
            .await?;

        Ok(serde_json::from_value(val)?)
    }

    pub async fn list(
        supa: &Supabase
    ) -> Result<Vec<Self>, SupabasicError> {

        let raw = supa
            .from(Self::table())
            .select("*")
            .execute()
            .await?;

        Ok(serde_json::from_value(raw)?)
    }

    pub async fn insert(
        supa: &Supabase,
        payload: &SimulationRow
    ) -> Result<Self, SupabasicError> {

        let raw = supa
            .from(Self::table())
            .insert(serde_json::json!([payload]))
            .select("*")
            .execute()
            .await?;

        let mut rows: Vec<Self> = serde_json::from_value(raw)?;
        rows.pop().ok_or_else(|| SupabasicError::Other("empty insert response".into()))
    }

    pub async fn update(
        supa: &Supabase,
        sim_id: SimulationId,
        payload: &serde_json::Value
    ) -> Result<Vec<Self>, SupabasicError> {

        let json_id = serde_json::to_value(&sim_id).unwrap();

        let raw = supa
            .from(Self::table())
            .eq("simulation_id", &json_id.to_string())
            .update(payload.clone())
            .select("*")
            .execute()
            .await?;

        Ok(serde_json::from_value(raw)?)
    }

    pub async fn patch(
        supa: &Supabase,
        sim_id: SimulationId,
        changes: &serde_json::Value
    ) -> Result<Vec<Self>, SupabasicError> {
        Self::update(supa, sim_id, changes).await
    }

    pub async fn delete(
        supa: &Supabase,
        sim_id: SimulationId
    ) -> Result<serde_json::Value, SupabasicError> {

        let json_id = serde_json::to_value(&sim_id).unwrap();

        let raw = supa
            .from(Self::table())
            .eq("simulation_id", &json_id.to_string())
            .delete()
            .execute()
            .await?;

        Ok(raw)
    }
}

// src/supabasic/simulations.rs
use crate::supabasic::client::Supabase;
use crate::supabasic::SupabasicError;
use crate::supabasic::orm::DbModel;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationRow {
    pub simulation_id: Uuid,
    pub user_owner_id: Option<Uuid>,
    pub anon_owner_id: Option<Uuid>,
    pub tick_rate: i64,
    pub world_id: i64,
    pub last_saved: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>, // ✅ Add this so reads & writes stay aligned
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateSimulation {
    pub world_id: Option<i64>,
    pub tick_rate: Option<i64>,
    pub anon_owner_id: Option<uuid::Uuid>,
    pub user_owner_id: Option<uuid::Uuid>,
    pub last_saved: Option<chrono::DateTime<chrono::Utc>>,
    pub metadata: Option<serde_json::Value>,
}

impl DbModel for SimulationRow {
    fn table() -> &'static str { "simulations" }
}

impl SimulationRow {
    /// GET by id (like get_user)
    pub async fn get(supa: &Supabase, sim_id: Uuid) -> Result<Self, SupabasicError> {
        let val = supa
            .from(Self::table())
            .select("*")
            .eq("simulation_id", &sim_id.to_string())
            .single()
            .await?;

        Ok(serde_json::from_value(val)?)
    }

    /// LIST all (like list_anon_users)
    pub async fn list(supa: &Supabase) -> Result<Vec<Self>, SupabasicError> {
        let raw = supa
            .from(Self::table())
            .select("*")
            .execute()
            .await?;

        Ok(serde_json::from_value(raw)?)
    }

    /// INSERT new row (like create_anon_user)
    pub async fn insert(supa: &Supabase, payload: &SimulationRow) -> Result<Self, SupabasicError> {
        let raw = supa
            .from(Self::table())
            .insert(serde_json::json!([payload]))
            .select("*")
            .execute()
            .await?;

        let inserted: Vec<Self> = serde_json::from_value(raw)?;
        inserted.into_iter().next().ok_or_else(|| {
            SupabasicError::Other("empty insert response".into())
        })
    }

    pub async fn update(
        supa: &Supabase,
        sim_id: Uuid,
        payload: &serde_json::Value
    ) -> Result<Vec<Self>, SupabasicError> {
        let raw = supa
            .from(Self::table())
            .eq("simulation_id", &sim_id.to_string())
            .update(payload.clone())
            .select("*")
            .execute()
            .await?;

        Ok(serde_json::from_value(raw)?)
    }

    pub async fn patch(
        supa: &Supabase,
        sim_id: Uuid,
        changes: &serde_json::Value
    ) -> Result<Vec<Self>, SupabasicError> {
        Self::update(supa, sim_id, changes).await
    }

    pub async fn delete(
        supa: &Supabase,
        sim_id: Uuid
    ) -> Result<serde_json::Value, SupabasicError> {
        let raw = supa
            .from(Self::table())
            .eq("simulation_id", &sim_id.to_string())
            .delete()
            .execute()
            .await?;

        Ok(raw)
    }
}

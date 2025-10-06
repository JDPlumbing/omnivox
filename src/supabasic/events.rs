use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;
use serde_json::to_string_pretty;

/// Mirrors your `events` table
#[derive(Debug, Serialize, Deserialize)]
pub struct EventRow {
    pub id: Uuid,
    pub simulation_id: Uuid,
    pub entity_id: Uuid,
    pub frame_id: i64,
    pub r_um: i64,
    pub lat_code: i64,
    pub lon_code: i64,
    pub ticks: i64,
    pub timestamp: DateTime<Utc>,
    pub kind: String,
    pub move_offset: Option<Value>,
    pub payload: Option<Value>,
    pub created_at: DateTime<Utc>,
}

impl DbModel for EventRow {
    fn table() -> &'static str { "events" }
}

impl EventRow {
    pub async fn list_for_sim(
        supa: &Supabase,
        sim_id: &Uuid,
    ) -> Result<Vec<Self>, SupabasicError> {
        supa.from(Self::table())
            .select("id,simulation_id,entity_id,frame_id,r_um,lat_code,lon_code,ticks,timestamp,kind,move_offset,payload,created_at")
            .eq("simulation_id", &sim_id.to_string())
            .execute_typed::<Self>()
            .await
    }
    pub async fn list_for_frame(
        supa: &Supabase,
        frame_id: i64,
    ) -> Result<Vec<Self>, SupabasicError> {
        supa.from(Self::table())
            .select("id,simulation_id,entity_id,frame_id,r_um,lat_code,lon_code,ticks,timestamp,kind,move_offset,payload,created_at")
            .eq("frame_id", &frame_id.to_string())
            .execute_typed::<Self>()
            .await
    }
    pub async fn list_for_entity(
        supa: &Supabase,
        entity_id: &Uuid,
    ) -> Result<Vec<Self>, SupabasicError> {
        supa.from(Self::table())
            .select("id,simulation_id,entity_id,frame_id,r_um,lat_code,lon_code,ticks,timestamp,kind,move_offset,payload,created_at")
            .eq("entity_id", &entity_id.to_string())
            .execute_typed::<Self>()
            .await
    }
    pub async fn get(supa: &Supabase, id: Uuid) -> Result<Self, SupabasicError> {
        supa.from(Self::table())
            .select("id,simulation_id,entity_id,frame_id,r_um,lat_code,lon_code,ticks,timestamp,kind,move_offset,payload,created_at")
            .eq("id", &id.to_string())
            .single_typed::<Self>()
            .await
    }

pub async fn create(supa: &Supabase, payload: &Self) -> Result<Self, SupabasicError> {
    let raw = supa.from("events")
        .insert(payload)
        .select("*")
        .execute()
        .await?;

    println!(
        "DEBUG Event insert raw response (JSON): {}",
        serde_json::to_string_pretty(&raw).unwrap_or_else(|_| "<invalid>".to_string())
    );

    // Try to parse anyway
    let inserted: Vec<Self> = serde_json::from_value(raw.clone())
        .map_err(|e| SupabasicError::Other(format!("decode error: {:?}, raw={}", e, raw)))?;

    inserted.into_iter().next()
        .ok_or_else(|| SupabasicError::Other("empty insert response".into()))
}

    
}


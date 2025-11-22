use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;
use chrono::{DateTime, Utc};

//use crate::core::chronovox::event::{ChronoEvent, EventKind};
//use crate::core::tdt::sim_time::SimTime;

/// ---------------------------------------------------------------------------
/// DB row schema (matches Supabase `events` table)
/// ---------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct EventRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,

    pub simulation_id: Uuid,
    pub entity_id: Uuid,
    pub world_id: i64,

    pub ticks: i128,              // SimTime as ns
    pub kind: String,             // serialized EventKind
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,   // extra data

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

impl DbModel for EventRow {
    fn table() -> &'static str { "events" }
}

impl EventRow {
    // -----------------------------------------------------------------------
    // CREATE
    // -----------------------------------------------------------------------
    pub async fn create(
        supa: &Supabase,
        payload: &Self
    ) -> Result<Self, SupabasicError> {

        let event_json = serde_json::to_value(payload)
            .expect("serialize EventRow");

        let raw = supa
            .from(Self::table())
            .insert_raw(serde_json::json!([event_json]))
            .select("*")
            .execute()
            .await?;

        let inserted: Vec<Self> = serde_json::from_value(raw.clone())
            .map_err(|e| SupabasicError::Other(format!("decode error: {:?}, raw={}", e, raw)))?;

        inserted
            .into_iter()
            .next()
            .ok_or_else(|| SupabasicError::Other("empty insert response".into()))
    }

    // -----------------------------------------------------------------------
    // LIST BY SIMULATION
    // -----------------------------------------------------------------------
    pub async fn list_for_sim(
        supa: &Supabase,
        sim_id: &Uuid
    ) -> Result<Vec<Self>, SupabasicError> {

        supa.from(Self::table())
            .select("*")
            .eq("simulation_id", &sim_id.to_string())
            .execute_typed::<Self>()
            .await
    }

    // -----------------------------------------------------------------------
    // LIST BY ENTITY
    // -----------------------------------------------------------------------
    pub async fn list_for_entity(
        supa: &Supabase,
        entity_id: &Uuid
    ) -> Result<Vec<Self>, SupabasicError> {

        supa.from(Self::table())
            .select("*")
            .eq("entity_id", &entity_id.to_string())
            .execute_typed::<Self>()
            .await
    }

    // -----------------------------------------------------------------------
    // LIST BY WORLD
    // -----------------------------------------------------------------------
    pub async fn list_for_world(
        supa: &Supabase,
        world_id: i64
    ) -> Result<Vec<Self>, SupabasicError> {

        supa.from(Self::table())
            .select("*")
            .eq("world_id", &world_id.to_string())
            .execute_typed::<Self>()
            .await
    }

    // -----------------------------------------------------------------------
    // GET SINGLE
    // -----------------------------------------------------------------------
    pub async fn get(
        supa: &Supabase,
        id: Uuid
    ) -> Result<Self, SupabasicError> {

        supa.from(Self::table())
            .select("*")
            .eq("id", &id.to_string())
            .single_typed::<Self>()
            .await
    }
}


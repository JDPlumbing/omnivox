use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;

/// Mirrors your `events` table
#[derive(Debug, Serialize, Deserialize)]
pub struct EventRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub simulation_id: Uuid,
    pub entity_id: Uuid,
    pub frame_id: i64,
    pub r_um: i64,
    pub lat_code: i64,
    pub lon_code: i64,
    pub ticks: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}



impl DbModel for EventRow {
    fn table() -> &'static str { "events" }
}

impl EventRow {
    // -----------------------------------
    // CREATE
    // -----------------------------------
pub async fn create(supa: &Supabase, payload: &Self) -> Result<Self, SupabasicError> {
    // 🧠 Debug: Print keys for this single event before insert
    let mut event_json = serde_json::to_value(payload)
        .unwrap_or_else(|_| serde_json::json!({ "error": "failed to serialize" }));

    // Ensure consistent keys across inserts
    if let Some(obj) = event_json.as_object_mut() {
        obj.entry("r_um").or_insert(Value::from(0));
        obj.entry("lat_code").or_insert(Value::from(0));
        obj.entry("lon_code").or_insert(Value::from(0));
        obj.entry("payload").or_insert(Value::Null);
        obj.entry("timestamp").or_insert(Value::Null);
        obj.entry("created_at").or_insert(Value::Null);
    }

    println!("Event keys (normalized): {:?}", event_json.as_object().unwrap().keys());

    let raw = supa
        .from("events")
        .insert_raw(serde_json::json!([event_json])) // use normalized event
        .select("*")
        .execute()
        .await?;


    println!(
        "DEBUG Event insert raw response (JSON): {}",
        serde_json::to_string_pretty(&raw).unwrap_or_else(|_| "<invalid>".to_string())
    );

    let inserted: Vec<Self> = serde_json::from_value(raw.clone())
        .map_err(|e| SupabasicError::Other(format!("decode error: {:?}, raw={}", e, raw)))?;

    inserted
        .into_iter()
        .next()
        .ok_or_else(|| SupabasicError::Other("empty insert response".into()))
}

    // -----------------------------------
    // LIST BY SIMULATION
    // -----------------------------------
    pub async fn list_for_sim(supa: &Supabase, sim_id: &Uuid) -> Result<Vec<Self>, SupabasicError> {
        supa.from(Self::table())
            .select("id,simulation_id,entity_id,frame_id,r_um,lat_code,lon_code,ticks,timestamp,kind,payload,created_at")
            .eq("simulation_id", &sim_id.to_string())
            .execute_typed::<Self>()
            .await
    }

    // -----------------------------------
    // LIST BY FRAME
    // -----------------------------------
pub async fn list_for_frame(supa: &Supabase, frame_id: i64) -> Result<Vec<Self>, SupabasicError> {
    supa.from(Self::table())
        .select("id,simulation_id,entity_id,frame_id,r_um,lat_code,lon_code,ticks,timestamp,kind,payload,created_at")
        .eq("frame_id", &frame_id.to_string())
        .execute_typed::<Self>()
        .await
}

    // -----------------------------------
    // LIST BY ENTITY
    // -----------------------------------
pub async fn list_for_entity(
    supa: &Supabase,
    entity_id: &Uuid,
) -> Result<Vec<Self>, SupabasicError> {
    supa.from(Self::table())
        .select("id,simulation_id,entity_id,frame_id,r_um,lat_code,lon_code,ticks,timestamp,kind,payload,created_at")
        .eq("entity_id", &entity_id.to_string())
        .execute_typed::<Self>()
        .await
}


    // -----------------------------------
    // GET SINGLE
    // -----------------------------------
    pub async fn get(supa: &Supabase, id: Uuid) -> Result<Self, SupabasicError> {
        supa.from(Self::table())
            .select("id,simulation_id,entity_id,frame_id,r_um,lat_code,lon_code,ticks,timestamp,kind,payload,created_at")
            .eq("id", &id.to_string())
            .single_typed::<Self>()
            .await
    }
}

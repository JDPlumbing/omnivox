// src/supabasic/worlds.rs
use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// Mirrors the `worlds` table in Supabase
#[derive(Debug, Serialize, Deserialize)]
pub struct WorldRow {
    pub frame_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl DbModel for WorldRow {
    fn table() -> &'static str { "worlds" }
}

impl WorldRow {
    pub async fn list(supa: &Supabase) -> Result<Vec<Self>, SupabasicError> {
        supa.from(Self::table())
            .select("frame_id,name,description,created_at,updated_at,deleted_at")
            .execute_typed::<Self>()
            .await
    }

    pub async fn get(supa: &Supabase, frame_id: i64) -> Result<Self, SupabasicError> {
        supa.from(Self::table())
            .select("frame_id,name,description,created_at,updated_at,deleted_at")
            .eq("frame_id", &frame_id.to_string())
            .single_typed::<Self>()
            .await
    }

    pub async fn create(supa: &Supabase, payload: &NewWorld) -> Result<Self, SupabasicError> {
        let inserted: Vec<Self> = supa
            .from(Self::table())
            .insert(payload)
            .select("frame_id,name,description,created_at,updated_at,deleted_at")
            .execute_typed()
            .await?;

        inserted.into_iter().next()
            .ok_or_else(|| SupabasicError::Other("empty insert response".into()))
    }
}

/// Payload used when creating a new world
#[derive(Debug, Serialize, Deserialize)]
pub struct NewWorld {
    pub frame_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}

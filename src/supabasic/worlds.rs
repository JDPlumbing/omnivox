// src/supabasic/worlds.rs

use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// Mirrors the `worlds` table in Supabase.
/// This contains ONLY persistent metadata.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldRecord {
    pub frame_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl DbModel for WorldRecord {
    fn table() -> &'static str { "worlds" }
}

impl WorldRecord {
    /// Get a list of all worlds.
    pub async fn list(supa: &Supabase) -> Result<Vec<Self>, SupabasicError> {
        supa.from(Self::table())
            .select("frame_id,name,description,created_at,updated_at,deleted_at")
            .execute_typed::<Self>()
            .await
    }

    /// Fetch a single world by its frame_id.
    pub async fn fetch(supa: &Supabase, frame_id: i64) -> Result<Self, SupabasicError> {
        supa.from(Self::table())
            .select("frame_id,name,description,created_at,updated_at,deleted_at")
            .eq("frame_id", &frame_id.to_string())
            .single_typed::<Self>()
            .await
    }

    /// Same as `fetch()` but preserves your old name.
    pub async fn get(supa: &Supabase, frame_id: i64) -> Result<Self, SupabasicError> {
        Self::fetch(supa, frame_id).await
    }

    /// Create a new world.
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

/// Payload for inserting a new world into Supabase.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewWorld {
    pub frame_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}

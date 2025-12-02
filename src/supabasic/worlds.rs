// src/supabasic/worlds.rs

use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;
use crate::core::id::WorldId;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// Mirrors the `worlds` table in Supabase.
/// Contains ONLY persistent metadata.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WorldRecord {
    pub world_id: WorldId,
    pub name: Option<String>,
    pub description: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub world_epoch: Option<i128>,  // raw i128 ns
}

impl DbModel for WorldRecord {
    fn table() -> &'static str { "worlds" }
}

impl WorldRecord {
    /// List all worlds
    pub async fn list(supa: &Supabase) -> Result<Vec<Self>, SupabasicError> {
        supa.from(Self::table())
            .select("world_id,name,description,world_epoch,created_at,updated_at,deleted_at")
            .execute_typed::<Self>()
            .await
    }

    /// Fetch world by world_id
    pub async fn fetch(supa: &Supabase, world_id: WorldId) -> Result<Self, SupabasicError> {
        supa.from(Self::table())
            .select("world_id,name,description,world_epoch,created_at,updated_at,deleted_at")
            .eq("world_id", &world_id.to_string())

            .single_typed::<Self>()
            .await
    }

    /// Alias for fetch()
    pub async fn get(supa: &Supabase, world_id: WorldId) -> Result<Self, SupabasicError> {
        Self::fetch(supa, world_id).await
    }

    /// Insert a new world
    pub async fn create(supa: &Supabase, payload: &NewWorld) -> Result<Self, SupabasicError> {
        let rows: Vec<Self> = supa
            .from(Self::table())
            .insert(payload)
            .select("world_id,name,description,world_epoch,created_at,updated_at,deleted_at")
            .execute_typed()
            .await?;

        rows.into_iter()
            .next()
            .ok_or_else(|| SupabasicError::Other("empty insert response".into()))
    }
}

/// Payload for creating new worlds
#[derive(Debug, Serialize, Deserialize)]
pub struct NewWorld {
    pub world_id: WorldId,
    pub name: Option<String>,
    pub description: Option<String>,
}
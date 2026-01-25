// src/supabasic/worlds.rs

use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;
use crate::core::id::WorldId;
use serde_json::Value;
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::core::world::world_env_descriptor::WorldEnvDescriptor;

/// Mirrors the `worlds` table in Supabase.
/// Contains ONLY persistent metadata.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WorldRow {
    pub world_id: WorldId,
    pub name: Option<String>,
    pub description: Option<String>,

    pub environment: Option<WorldEnvDescriptor>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub world_epoch: Option<String>,  // raw i128 ns
}

impl DbModel for WorldRow {
    fn table() -> &'static str { "worlds" }
}

impl WorldRow {
    /// List all worlds
    pub async fn list(supa: &Supabase) -> Result<Vec<Self>, SupabasicError> {
        supa.from(Self::table())
            .select("world_id,name,description,environment,world_epoch,created_at,updated_at,deleted_at")
            .execute_typed::<Self>()
            .await
    }

    /// Fetch world by world_id
    pub async fn fetch(supa: &Supabase, world_id: WorldId) -> Result<Self, SupabasicError> {
        supa.from(Self::table())
            .select("world_id,name,description,environment,world_epoch,created_at,updated_at,deleted_at")
            .eq("world_id", &world_id.to_string())

            .single_typed::<Self>()
            .await
    }

    /// Alias for fetch()
    pub async fn get(supa: &Supabase, world_id: WorldId) -> Result<Self, SupabasicError> {
        Self::fetch(supa, world_id).await
    }

    /// Insert a new world
    pub async fn create(supa: &Supabase, payload: &NewWorldRow) -> Result<Self, SupabasicError> {
        let rows: Vec<Self> = supa
            .from(Self::table())
            .insert(payload)
            .select("world_id,name,description,environment,world_epoch,created_at,updated_at,deleted_at")
            .execute_typed()
            .await?;

        rows.into_iter()
            .next()
            .ok_or_else(|| SupabasicError::Other("empty insert response".into()))
    }

    /// Delete a world by world_id
    pub async fn delete(supa: &Supabase, world_id: WorldId) -> Result<(), SupabasicError> {
        supa.from(Self::table())
            .delete()
            .eq("world_id", &world_id.to_string())
            .execute()
            .await?;

        Ok(())
    }
}

/// Payload for creating new worlds
#[derive(Debug, Serialize, Deserialize)]
pub struct NewWorldRow {
    
    pub name: Option<String>,
    pub description: Option<String>,
    pub world_epoch: Option<String>,  // raw i128 ns
    pub environment: serde_json::Value,
}
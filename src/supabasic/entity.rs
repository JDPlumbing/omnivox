// src/supabasic/entity.rs

use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;

//use crate::core::uvoxid::UvoxId;
use crate::sim::entity::SimEntity;
use crate::core::SimTime;


/// ---------------------------------------------------------------------------
/// Mirrors the `sim_entities` table in Supabase.
/// Blueprints + uvoxid are stored **inline** as JSON.
/// ---------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityRecord {
    pub entity_id: Uuid,

    pub world_id: i64,

    pub blueprint: Value,   // inline Objex
    pub uvoxid: Value,      // inline UvoxId
    pub orientation: Value, // inline UvoxQuat

    pub spawned_at: i128,
    pub despawned_at: Option<i128>,

    pub metadata: Value,
}

impl DbModel for EntityRecord {
    fn table() -> &'static str { "sim_entities" }
}

//---------------------------------------------------------------------------
// Conversions
// ---------------------------------------------------------------------------

impl From<&SimEntity> for EntityRecord {
    fn from(e: &SimEntity) -> Self {
        EntityRecord {
            entity_id: e.entity_id,

            world_id: e.world_id,

            blueprint: serde_json::to_value(&e.blueprint).unwrap(),
            uvoxid: serde_json::to_value(&e.uvoxid).unwrap(),
            orientation: serde_json::to_value(&e.orientation).unwrap(),

            spawned_at: e.spawned_at.as_ns(),
            despawned_at: e.despawned_at.map(|t| t.as_ns()),

            metadata: e.metadata.clone(),
        }
    }
}

impl TryFrom<EntityRecord> for SimEntity {
    type Error = serde_json::Error;

    fn try_from(r: EntityRecord) -> Result<Self, Self::Error> {
        Ok(SimEntity {
            entity_id: r.entity_id,
            world_id: r.world_id,

            blueprint: serde_json::from_value(r.blueprint)?,
            uvoxid: serde_json::from_value(r.uvoxid)?,
            orientation: serde_json::from_value(r.orientation)?,

            spawned_at: SimTime::from_ns(r.spawned_at),
            despawned_at: r.despawned_at.map(SimTime::from_ns),

            metadata: r.metadata,
        })
    }
}

//---------------------------------------------------------------------------
// CRUD Helpers
// ---------------------------------------------------------------------------

impl EntityRecord {
    /// Insert SimEntity → DB
    pub async fn insert(
        supa: &Supabase,
        entity: &SimEntity,
    ) -> Result<Self, SupabasicError> {

        // Insert expects a JSON array
        let payload = serde_json::json!([EntityRecord::from(entity)]);

        let raw = supa
            .from(Self::table())
            .insert(payload)
            .select("*")
            .execute()
            .await?;

        let mut rows: Vec<Self> =
            serde_json::from_value(raw.clone())
                .map_err(|e| SupabasicError::Other(format!(
                    "decode error: {e:?}, raw={raw}"
                )))?;

        Ok(rows.remove(0))
    }

    /// Fetch 1 entity by UUID
    pub async fn fetch(supa: &Supabase, id: Uuid)
        -> Result<Self, SupabasicError>
    {
        supa.from(Self::table())
            .select("*")
            .eq("entity_id", &id.to_string())
            .single_typed()
            .await
    }

    /// List all entities in a world
    pub async fn list_for_world(
        supa: &Supabase,
        world_id: i64,
    ) -> Result<Vec<Self>, SupabasicError> {

        supa.from(Self::table())
            .select("*")
            .eq("world_id", &world_id.to_string())
            .execute_typed::<Self>()
            .await
    }
}

// src/supabasic/entity.rs

use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;
use crate::core::id::{EntityId, WorldId};
use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;

//use crate::core::uvoxid::UvoxId;
use crate::engine::entity::SimEntity;
use crate::core::SimTime;
use crate::core::sim_time::{ deserialize_simtime, deserialize_simtime_opt } ;


/// ---------------------------------------------------------------------------
/// Mirrors the `sim_entities` table in Supabase.
/// objexs + uvoxid are stored **inline** as JSON.
/// ---------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_id: Option<Uuid>,   // ← // DB identity ONLY

    pub world_id: WorldId,

    pub template: Value,
    pub position: Value,
    pub orientation: Value,

    #[serde(deserialize_with = "deserialize_simtime")]
    pub spawned_at: SimTime,

    #[serde(deserialize_with = "deserialize_simtime_opt")]
    pub despawned_at: Option<SimTime>,

    pub metadata: Value,
}


impl DbModel for EntityRow {
    fn table() -> &'static str { "sim_entities" }
}

//---------------------------------------------------------------------------
// Conversions
// ---------------------------------------------------------------------------

impl From<&SimEntity> for EntityRow {
    fn from(e: &SimEntity) -> Self {
        EntityRow {
            
            row_id: None, // ← let Postgres generate it

            world_id: e.world_id,

            template: serde_json::to_value(&e.template).unwrap(),
            position: serde_json::to_value(&e.position).unwrap(),
            orientation: serde_json::to_value(&e.orientation).unwrap(),

            spawned_at: e.spawned_at,
            despawned_at: e.despawned_at,

            metadata: e.metadata.clone(),
        }
    }
}
impl TryFrom<EntityRow> for SimEntity {
    type Error = serde_json::Error;

    fn try_from(r: EntityRow) -> Result<Self, Self::Error> {
        Ok(SimEntity {
            id: EntityId::provisional(0), // or allocator.next()

            world_id: r.world_id,

            template: serde_json::from_value(r.template)?,
            position: serde_json::from_value(r.position)?,
            orientation: serde_json::from_value(r.orientation)?,

            spawned_at: r.spawned_at,
            despawned_at: r.despawned_at,

            metadata: r.metadata,
        })
    }
}

//---------------------------------------------------------------------------
// CRUD Helpers
// ---------------------------------------------------------------------------

impl EntityRow {
    /// Insert SimEntity → DB
    pub async fn insert(
        supa: &Supabase,
        entity: &SimEntity,
    ) -> Result<Self, SupabasicError> {

        // Insert expects a JSON array
        let payload = serde_json::json!([EntityRow::from(entity)]);

        let raw = supa
            .from(Self::table())
            .insert_raw(payload)
            
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
            .eq("row_id", &id.to_string())

            .single_typed()
            .await
    }

    /// List all entities in a world
    pub async fn list_for_world(
        supa: &Supabase,
        world_id: WorldId,
    ) -> Result<Vec<Self>, SupabasicError> {

        supa.from(Self::table())
            .select("*")
            .eq("world_id", &world_id.to_string())
            .execute_typed::<Self>()
            .await
    }
}

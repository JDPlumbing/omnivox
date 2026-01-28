use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use crate::core::id::{EntityId, WorldId};
use crate::core::SimTime;


use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;



/// ---------------------------------------------------------------------------
/// Mirrors the `sim_entities` table in Supabase.
/// This is a *persistence DTO*, not a runtime entity.
/// ---------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityRow {
    /// Primary key (EntityId)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_id: Option<Uuid>,

    pub world_id: WorldId,

    /// Serialized components
    //pub objex_template_id: Uuid,
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
// CRUD helpers
//---------------------------------------------------------------------------

impl EntityRow {
    /// Insert a new entity row (DB assigns UUID)
    pub async fn insert(
        supa: &Supabase,
        row: &EntityRow,
    ) -> Result<Self, SupabasicError> {

        let payload = serde_json::json!([row]);

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

    /// Fetch a single entity by EntityId
    pub async fn fetch(
        supa: &Supabase,
        id: EntityId,
    ) -> Result<Self, SupabasicError> {
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

// src/supabasic/entity.rs
use serde::{Serialize, Deserialize};
use serde_json::json;
use uuid::Uuid;

use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;

use crate::sim::entity::SimEntity;
use crate::uvoxid::UvoxId;
use crate::sim::time::SimTime;
use crate::sim::UvoxQuat;
use crate::core::MatCatId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityRecord {
    pub entity_id: Uuid,
    pub blueprint_id: Uuid,

    pub frame_id: i64,
    pub r_um: i64,
    pub lat_code: i64,
    pub lon_code: i64,

    pub orientation: serde_json::Value, // JSON quaternion

    pub spawned_at: i64,                // ns
    pub despawned_at: Option<i64>,      // ns

    pub metadata: serde_json::Value,
}

impl DbModel for EntityRecord {
    fn table() -> &'static str { "sim_entities" }
}

impl EntityRecord {
    pub async fn insert(
        supa: &Supabase,
        entity: &SimEntity,
        blueprint_id: Uuid
    ) -> Result<Self, SupabasicError>
    {
        let payload = json!({
            "entity_id": entity.entity_id,
            "blueprint_id": blueprint_id,

            "frame_id": entity.uvoxid.frame_id,
            "r_um": entity.uvoxid.r_um,
            "lat_code": entity.uvoxid.lat_code,
            "lon_code": entity.uvoxid.lon_code,

            "orientation": serde_json::to_value(&entity.orientation).unwrap(),

            "spawned_at": entity.spawned_at.as_ns(),
            "despawned_at": entity.despawned_at.map(|t| t.as_ns()),

            "metadata": entity.metadata,
        });

        let raw = supa
            .from(Self::table())
            .insert(payload)
            .select("*")
            .execute()
            .await?;

        let mut rows: Vec<Self> =
            serde_json::from_value(raw.clone())
                .map_err(|e| SupabasicError::Other(format!("decode error: {e:?}, raw={raw}")))?;

        Ok(rows.remove(0))
    }
}


    pub async fn fetch(supa: &Supabase, id: Uuid) -> Result<Self, SupabasicError> {
        supa.from(Self::table())
            .select("*")
            .eq("entity_id", id.to_string())
            .single_typed()
            .await
    }

impl EntityRecord {
    pub fn into_sim_entity(self, blueprint: Objex) -> SimEntity {
        SimEntity {
            entity_id: self.entity_id,
            blueprint,

            uvoxid: UvoxId::new(self.frame_id, self.r_um, self.lat_code, self.lon_code),

            orientation: serde_json::from_value(self.orientation)
                .unwrap_or_else(|_| UvoxQuat::identity()),

            spawned_at: SimTime::from_ns(self.spawned_at),
            despawned_at: self.despawned_at.map(SimTime::from_ns),

            metadata: self.metadata,
        }
    }
}


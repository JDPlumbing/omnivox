// src/supabasic/objex.rs
use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;
use crate::objex::{Objex, Shape, MaterialLink};
use crate::objex::core::{MaterialName, MaterialKind};

use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;
use serde_json::to_string_pretty;

/// Mirrors your `objex_entities` table
#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRecord {
    pub frame_id: i64,               // 🔥 include this
    pub entity_id: Option<Uuid>,     // DB may autogen
    pub name: String,
    pub shape: Value,                // stored as JSONB
    pub material_name: String,       // enums come back as text
    pub material_kind: String,       // enums come back as text
}

impl DbModel for ObjectRecord {
    fn table() -> &'static str { "objex_entities" }
}

impl ObjectRecord {
    pub async fn create(supa: &Supabase, payload: &Self) -> Result<Self, SupabasicError> {
        let raw = supa.from("objex_entities")
            .insert(payload)
            .select("*")
            .execute()
            .await?;

            println!(
                "DEBUG Objex insert raw response: {}",
                to_string_pretty(&raw).unwrap_or_else(|_| "<invalid json>".to_string())
            );

        // Now decode into typed rows
        let inserted: Vec<Self> = supa.from("objex_entities")
            .insert(payload)
            .select("frame_id,entity_id,name,shape,material_name,material_kind")
            .execute_typed()
            .await?;

        inserted.into_iter().next()
            .ok_or_else(|| SupabasicError::Other("empty insert response".into()))
    }


    pub async fn list(supa: &Supabase) -> Result<Vec<Self>, SupabasicError> {
        supa.from(Self::table())
            .select("frame_id,entity_id,name,shape,material_name,material_kind")
            .execute_typed::<Self>()
            .await
    }

    pub async fn get(supa: &Supabase, id: Uuid) -> Result<Self, SupabasicError> {
        supa.from(Self::table())
            .select("frame_id,entity_id,name,shape,material_name,material_kind")
            .eq("entity_id", &id.to_string())
            .single_typed::<Self>()
            .await
    }
}

// -------------------------
// Conversions
// -------------------------

impl From<Objex> for ObjectRecord {
    fn from(o: Objex) -> Self {
        ObjectRecord {
            frame_id: o.frame_id, // 🔥 include this
            entity_id: Some(o.entity_id), // wrap Uuid
            name: o.name,
            shape: serde_json::to_value(o.shape).unwrap(),
            material_name: format!("{:?}", o.material.name),
            material_kind: format!("{:?}", o.material.kind),
        }
    }
}

impl TryFrom<ObjectRecord> for Objex {
    type Error = anyhow::Error;

    fn try_from(r: ObjectRecord) -> Result<Self, Self::Error> {
        Ok(Objex {
            frame_id: r.frame_id, // 🔥 include this
            entity_id: r.entity_id.ok_or_else(|| anyhow::anyhow!("missing entity_id"))?,
            name: r.name,
            shape: serde_json::from_value(r.shape)?,
            material: MaterialLink {
                id: Uuid::new_v4(), // lightweight ref, can regen
                name: match r.material_name.as_str() {
                    "Concrete" => MaterialName::Concrete,
                    "Steel" => MaterialName::Steel,
                    "Copper" => MaterialName::Copper,
                    "Aluminum" => MaterialName::Aluminum,
                    "Wood" => MaterialName::Wood,
                    "Plastic" => MaterialName::Plastic,
                    "Rubber" => MaterialName::Rubber,
                    "Glass" => MaterialName::Glass,
                    "Air" => MaterialName::Air,
                    "Water" => MaterialName::Water,
                    other => MaterialName::Custom(other.to_string()),
                },
                kind: match r.material_kind.as_str() {
                    "Metal" => MaterialKind::Metal,
                    "Ceramic" => MaterialKind::Ceramic,
                    "Polymer" => MaterialKind::Polymer,
                    "Organic" => MaterialKind::Organic,
                    "Masonry" => MaterialKind::Masonry,
                    "Glass" => MaterialKind::Glass,
                    "Liquid" => MaterialKind::Liquid,
                    "Gas" => MaterialKind::Gas,
                    "Composite" => MaterialKind::Composite,
                    _ => MaterialKind::Other,
                },
            },
        })
    }
}

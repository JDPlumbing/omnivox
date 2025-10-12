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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObjectRecord {
    pub entity_id: Option<Uuid>,     // PK
    pub property_id: Option<Uuid>,   // FK → properties
    pub frame_id: i64,               // FK → worlds
    pub name: String,
    pub shape: Value,                // JSONB blob
    pub material_name: String,       // stringified enum
    pub material_kind: String,       // stringified enum
}


impl DbModel for ObjectRecord {
    fn table() -> &'static str { "objex_entities" }
}

impl ObjectRecord {
pub async fn create(supa: &Supabase, payload: &Self) -> Result<Self, SupabasicError> {
    use serde_json::json;

    let insert_payload = json!({
        "entity_id": payload.entity_id.unwrap_or_else(Uuid::new_v4),
        "property_id": payload.property_id,
        "name": payload.name,
        "shape": payload.shape,
        "material_name": payload.material_name,
        "material_kind": payload.material_kind,
        "frame_id": payload.frame_id
    });


    println!(
        "🧩 FINAL OBJEX INSERT PAYLOAD:\n{}",
        to_string_pretty(&insert_payload).unwrap()
    );

    let raw = supa
        .from(Self::table())
        .insert(insert_payload)
        .select("*")
        .execute()
        .await?;

    let inserted: Vec<Self> = serde_json::from_value(raw.clone())
        .map_err(|e| SupabasicError::Other(format!("decode error: {e:?}, raw={raw}")))?;

    inserted
        .into_iter()
        .next()
        .ok_or_else(|| SupabasicError::Other("empty insert response".into()))
}

pub async fn create_many(supa: &Supabase, payloads: &[Self]) -> Result<Vec<Self>, SupabasicError> {
    use serde_json::json;

    let json_array: serde_json::Value = serde_json::to_value(payloads)
        .map_err(|e| SupabasicError::Other(format!("serialization error: {e:?}")))?;

    let raw = supa
        .from(Self::table())
        .insert_raw(json_array)
        .select("*")
        .execute()
        .await?;

    let inserted: Vec<Self> = serde_json::from_value(raw.clone())
        .map_err(|e| SupabasicError::Other(format!("decode error: {e:?}, raw={raw}")))?;

    Ok(inserted)
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
            entity_id: Some(o.entity_id),
            property_id: o.property_id, // 👈 new
            frame_id: o.frame_id,
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
            property_id: r.property_id,

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

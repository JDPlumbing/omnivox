use crate::supabasic::Supabase;
use uuid::Uuid;
use serde_json::json;

use crate::objex::{Objex, MaterialLink};
use crate::objex::error::ObjexError;

pub async fn insert_objex(
    supa: &Supabase,
    objex: &Objex,
) -> Result<(), ObjexError> {
    supa.from("objex_entities")
        .insert(json!([{
            "frame_id": objex.frame_id,
            "entity_id": objex.entity_id,
            "name": objex.name,
            "shape": objex.shape,
            "material_name": format!("{:?}", objex.material.name),
            "material_kind": format!("{:?}", objex.material.kind),
        }]))
        .execute()
        .await?;


    Ok(())
}

/// Fetch an Objex_entity
pub async fn fetch_objex(
    supa: &Supabase,
    entity_id: Uuid,
) -> Result<Objex, ObjexError> {
    let val: serde_json::Value = supa
        .from("objex_entities")
        .select("frame_id,entity_id, name, shape, material_name, material_kind")
        .eq("entity_id", &entity_id.to_string())
        .single()
        .await?;

    Ok(Objex {
        frame_id: val["frame_id"].as_i64().unwrap_or(0), // 🔥 include this
        entity_id,
        name: val["name"].as_str().unwrap_or("Unnamed Objex").to_string(),
        shape: serde_json::from_value(val["shape"].clone())?,
        material: MaterialLink::new(
            serde_json::from_value(val["material_name"].clone())?
        ),
    })
}

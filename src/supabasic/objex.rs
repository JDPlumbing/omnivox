// src/supabasic/objex.rs

use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;

use crate::core::objex::Objex;
use crate::core::objex::core::material::{MaterialLink, MaterialName, MaterialKind};
use crate::core::objex::geospec::shapes::Shape;

use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

/// ---------------------------------------------------------------------------
/// Mirrors the `objex` table in Supabase
/// Canonical blueprint storage: shape + material ONLY
/// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjexRecord {
    pub shape: Value,            // JSON encoded Shape
    pub material_name: String,
    pub material_kind: String,

    pub matcat_category: u8,
    pub matcat_variant: u16,
    pub matcat_grade: u16,
}

impl DbModel for ObjexRecord {
    fn table() -> &'static str { "objex" }
}


impl ObjexRecord {
    /// Insert one blueprint
    pub async fn insert(supa: &Supabase, bp: &Objex) -> Result<Self, SupabasicError> {
        let payload = Self::from(bp);

        let raw = supa
            .from(Self::table())
            .insert(&payload)
            .select("*")
            .execute()
            .await?;

        // should return exactly one
        let mut rows: Vec<ObjexRecord> =
            serde_json::from_value(raw.clone())
                .map_err(|e| SupabasicError::Other(format!("decode error: {e:?}, raw={raw}")))?;

        Ok(rows.remove(0))
    }

    /// Get by exact shape+material combination
    pub async fn get_exact(
        supa: &Supabase,
        shape: &Value,
        material_name: &str,
        material_kind: &str
    ) -> Result<Self, SupabasicError> {

        supa.from(Self::table())
            .select("*")
            .eq("material_name", material_name)
            .eq("material_kind", material_kind)
            .eq("shape", &shape.to_string()) // JSON equality match
            .single_typed()
            .await
    }

    /// List all canonical blueprints
    pub async fn list(supa: &Supabase) -> Result<Vec<Self>, SupabasicError> {
        supa.from(Self::table())
            .select("*")
            .execute_typed::<Self>()
            .await
    }
}


/// ---------------------------------------------------------------------------
/// Conversions
/// ---------------------------------------------------------------------------

impl From<&Objex> for ObjexRecord {
    fn from(o: &Objex) -> Self {
        ObjexRecord {
            shape: serde_json::to_value(&o.shape).unwrap(),
            material_name: format!("{:?}", o.material.name),
            material_kind: format!("{:?}", o.material.kind),

            matcat_category: o.material.matcat_id.category,
            matcat_variant: o.material.matcat_id.variant,
            matcat_grade: o.material.matcat_id.grade,
        }
    }
}

impl TryFrom<ObjexRecord> for Objex {
    type Error = anyhow::Error;

    fn try_from(r: ObjexRecord) -> Result<Self, Self::Error> {
        let mname = MaterialName::from_str(&r.material_name)
            .ok_or_else(|| anyhow::anyhow!("Invalid material_name"))?;

        let mkind = MaterialKind::from_str(&r.material_kind)
            .ok_or_else(|| anyhow::anyhow!("Invalid material_kind"))?;

        let matcat = MatCatId::new(r.matcat_category, r.matcat_variant, r.matcat_grade);

        Ok(Objex {
            shape: serde_json::from_value(r.shape)?,
            material: MaterialLink { name: mname, kind: mkind, matcat_id: matcat },
        })
    }
}


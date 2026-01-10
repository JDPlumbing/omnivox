use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use crate::supabasic::{Supabase, SupabasicError};


/// ─────────────────────────────────────────────
/// DB row (exact table shape)
/// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjexRow {
    pub id: Uuid,
    pub geospec_id: Uuid,
    pub matcat_category: u8,
    pub matcat_variant: Option<u16>,
    pub matcat_grade: Option<u16>,
    pub metadata: Value,
}

/// Used ONLY for inserts
#[derive(Debug, Clone, Serialize)]
pub struct NewObjexRow {
    pub geospec_id: Uuid,
    pub matcat_category: u8,
    pub matcat_variant: Option<u16>,
    pub matcat_grade: Option<u16>,
    pub metadata: Value,
}

impl Supabase {
    /// SELECT * FROM objex_templates
    pub async fn select_objex_templates(
        &self,
    ) -> Result<Vec<ObjexRow>, SupabasicError> {
        let value = self
            .from("objex_templates")
            .select("*")
            .execute()
            .await?;

        serde_json::from_value(value)
            .map_err(SupabasicError::from)
    }

    /// INSERT INTO objex_templates (...)
    pub async fn insert_objex_template(
        &self,
        row: NewObjexRow,
    ) -> Result<ObjexRow, SupabasicError> {
        let value = self
            .from("objex_templates")
            .insert(row)
            .single()   // ← THIS already executes
            .await?;

        serde_json::from_value(value)
            .map_err(SupabasicError::from)
    }
}

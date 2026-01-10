use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::shared::app_state::AppState;
use crate::supabasic::objex::objex::{ObjexRow, NewObjexRow};
use crate::core::objex::matcat::{MatCatId, CategoryId, VariantId, GradeId};


/// ─────────────────────────────────────────────
/// HTTP DTOs
/// ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateObjexRequest {
    pub geospec_id: Uuid,

    pub matcat_category: u8,
    pub matcat_variant: Option<u16>,
    pub matcat_grade: Option<u16>,

    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct ObjexResponse {
    pub id: Uuid,
    pub geospec_id: Uuid,
    pub matcat: MatCatId,
    pub metadata: Value,
}


/// ─────────────────────────────────────────────
/// Router
/// ─────────────────────────────────────────────

pub fn objex_routes() -> Router<AppState> {
    Router::new()
        .route("/templates", get(list_objex_templates).post(create_objex_template))
}


/// ─────────────────────────────────────────────
/// Handlers
/// ─────────────────────────────────────────────

async fn list_objex_templates(
    State(state): State<AppState>,
) -> Result<Json<Vec<ObjexResponse>>, String> {
    let rows = state
        .supa
        .select_objex_templates()
        .await
        .map_err(|e| e.to_string())?;

    let resp: Vec<ObjexResponse> = rows.into_iter().map(Into::into).collect();

    Ok(Json(resp))
}

async fn create_objex_template(
    State(state): State<AppState>,
    Json(req): Json<CreateObjexRequest>,
) -> Result<Json<ObjexResponse>, String> {
    let new_row = NewObjexRow {
        geospec_id: req.geospec_id,
        matcat_category: req.matcat_category,
        matcat_variant: req.matcat_variant,
        matcat_grade: req.matcat_grade,
        metadata: req.metadata.unwrap_or_else(|| Value::Object(Default::default())),
    };

    let row = state
        .supa
        .insert_objex_template(new_row)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(row.into()))
}


/// ─────────────────────────────────────────────
/// Conversions
/// ─────────────────────────────────────────────

impl From<ObjexRow> for ObjexResponse {
    fn from(row: ObjexRow) -> Self {
        Self {
            id: row.id,
            geospec_id: row.geospec_id,
            matcat: MatCatId {
                category: CategoryId(row.matcat_category),
                variant: row.matcat_variant.map(VariantId),
                grade: row.matcat_grade.map(GradeId),
            },
            metadata: row.metadata,
        }
    }
}

pub mod materials;
pub mod templates;
pub mod geospec;
pub mod objex;

use axum::{
    routing::{get, post},
    extract::State,
    Json,
    Router,
};

use crate::shared::AppState;
use crate::api::objex::objex::{CreateObjexRequest, ObjexResponse};
use crate::supabasic::objex::objex::{NewObjexRow, ObjexRow};
use crate::core::objex::matcat::{CategoryId, VariantId, GradeId, MatCatId};


/// ─────────────────────────────────────────────
/// Router
/// ─────────────────────────────────────────────

pub fn objex_template_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/templates",
            get(list_objex_templates).post(create_objex_template),
        )
}


/// ─────────────────────────────────────────────
/// Handlers
/// ─────────────────────────────────────────────

async fn list_objex_templates(
    State(state): State<AppState>,
) -> Json<Vec<ObjexResponse>> {
    let rows: Vec<ObjexRow> = state
        .supa
        .select_objex_templates()
        .await
        .unwrap_or_default();

    let resp: Vec<ObjexResponse> =
        rows.into_iter().map(Into::into).collect();

    Json(resp)
}

async fn create_objex_template(
    State(state): State<AppState>,
    Json(req): Json<CreateObjexRequest>,
) -> Json<ObjexResponse> {
    let new_row = NewObjexRow {
        geospec_id: req.geospec_id,
        matcat_category: req.matcat_category,
        matcat_variant: req.matcat_variant,
        matcat_grade: req.matcat_grade,
        metadata: req.metadata.unwrap_or_default(),
    };

    let created: ObjexRow = state
        .supa
        .insert_objex_template(new_row)
        .await
        .expect("failed to create objex template");

    Json(created.into())
}



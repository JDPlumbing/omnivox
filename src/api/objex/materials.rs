use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};

use serde::Deserialize;

use crate::shared::app_state::AppState;
use crate::core::objex::matcat::api::{
    get_categories,
    get_variants,
    get_grades,
    resolve_material,
    preview_material,
    IdName,
    ResolveResponse,
    PreviewResponse,
};

#[derive(Debug, Deserialize)]
pub struct ResolveQuery {
    pub variant: Option<u16>,
    pub grade: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct PreviewQuery {
    pub variant: Option<u16>,
    pub grade: Option<u16>,
}

async fn list_categories() -> Json<Vec<IdName>> {
    Json(get_categories())
}

async fn list_variants(
    Path(category): Path<u8>,
) -> Json<Vec<IdName>> {
    Json(get_variants(category))
}

async fn list_grades(
    Path((category, variant)): Path<(u8, u16)>,
) -> Json<Vec<IdName>> {
    Json(get_grades(category, variant))
}

async fn resolve(
    Path(category): Path<u8>,
    Query(q): Query<ResolveQuery>,
) -> Json<ResolveResponse> {
    Json(resolve_material(category, q.variant, q.grade))
}

async fn preview(
    Path(category): Path<u8>,
    Query(q): Query<PreviewQuery>,
) -> Json<PreviewResponse> {
    Json(preview_material(category, q.variant, q.grade))
}


pub fn material_routes() -> Router<AppState> {
    Router::new()
        .route("/categories", get(list_categories))
        .route("/categories/{category}/variants", get(list_variants))
        .route(
            "/categories/{category}/variants/{variant}/grades",
            get(list_grades),
        )
        .route("/resolve/{category}", get(resolve))
        .route("/preview/{category}", get(preview))
}

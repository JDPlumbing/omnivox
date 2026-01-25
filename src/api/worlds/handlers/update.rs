use axum::{extract::{Path, State}, response::IntoResponse, Json};
use axum::http::StatusCode;
use serde_json::json;

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;

pub async fn update_world_handler(
    State(app): State<AppState>,
    Path(world_id): Path<WorldId>,
    Json(changes): Json<serde_json::Value>,
) -> impl IntoResponse {
    match app.world_source.update_world(world_id, changes).await {
        Ok(row) => Json(row).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("{e:?}") })),
        ).into_response(),
    }
}

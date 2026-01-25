use axum::{extract::{Path, State}, response::IntoResponse, Json};
use axum::http::StatusCode;
use serde_json::json;

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::api::worlds::dtos::world::WorldDto;

pub async fn get_world_handler(
    State(app): State<AppState>,
    Path(world_id): Path<WorldId>,
) -> impl IntoResponse {
    match app.world_source.get_world(world_id).await {
        Ok(row) => Json(WorldDto::from_row(&row)).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "world not found" })),
        ).into_response(),
    }
}

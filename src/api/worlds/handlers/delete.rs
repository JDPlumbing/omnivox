use axum::{extract::{Path, State}, response::IntoResponse, Json};
use axum::http::StatusCode;

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;

pub async fn delete_world_handler(
    State(app): State<AppState>,
    Path(world_id): Path<WorldId>,
) -> impl IntoResponse {
    match app.world_source.delete_world(world_id).await {
        Ok(_) => Json("deleted").into_response(),
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

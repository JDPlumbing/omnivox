use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use crate::shared::app_state::AppState;
use crate::core::WorldId;
use reqwest::StatusCode;

pub async fn list_world_properties(
    State(state): State<AppState>,
    Path(world_id): Path<WorldId>,
) -> impl IntoResponse {
    match state
        .property_source
        .list_for_world(world_id)
        .await
    {
        Ok(props) => Json(props).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

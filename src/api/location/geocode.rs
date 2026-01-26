use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;
use crate::shared::app_state::AppState;
use reqwest::StatusCode;

pub async fn resolve_address(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.location_source.resolve_address(id).await {
        Ok(loc) => Json(loc).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

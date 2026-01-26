use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use serde_json::json;
use uuid::Uuid;

use crate::shared::app_state::AppState;

pub async fn delete_property(
    State(state): State<AppState>,
    Path(property_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.property_source.delete(property_id).await {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({ "deleted": property_id })),
        )
            .into_response(),

        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

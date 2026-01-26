use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use serde_json::json;
use uuid::Uuid;

use crate::shared::app_state::AppState;

pub async fn get_property(
    State(state): State<AppState>,
    Path(property_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.property_source.get(property_id).await {
        Ok(Some(property)) => Json(property).into_response(),

        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "property not found" })),
        )
            .into_response(),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

use axum::{
    extract::State,
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use serde_json::json;

use crate::shared::app_state::AppState;

pub async fn list_properties(
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.property_source.list_all().await {
        Ok(properties) => Json(properties).into_response(),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

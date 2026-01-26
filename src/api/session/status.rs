use axum::{
    extract::{State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::shared::app_state::AppState;

pub async fn session_status(
    State(app): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let Some(session_id) = headers
        .get("x-session-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
    else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    match app.session_source.get_session(session_id).await {
        Ok(session) => Json(json!({ "session": session })).into_response(),
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}

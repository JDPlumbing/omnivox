use axum::{
    extract::{State, Json},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::shared::app_state::AppState;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(app): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    let Some(session_id) = headers
        .get("x-session-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
    else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    match app
        .user_engine
        .login(session_id, req.email, req.password)
        .await
    {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            tracing::warn!("login failed: {}", e);
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}

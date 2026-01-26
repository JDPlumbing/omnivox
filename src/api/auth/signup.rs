use axum::{
    extract::{State, Json},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::shared::app_state::AppState;

#[derive(Deserialize)]
pub struct SignupReq {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

pub async fn signup(
    State(app): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<SignupReq>,
) -> impl IntoResponse {
    let Some(session_id) = headers
        .get("x-session-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
    else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

match app.user_engine
    .signup(session_id, req.email, req.password, req.display_name)
    .await
{
    Ok(_) => StatusCode::OK.into_response(),
    Err(e) => {
        tracing::warn!("signup failed: {}", e);
        StatusCode::BAD_REQUEST.into_response()
    }
}

}

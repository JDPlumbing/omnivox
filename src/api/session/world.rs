use axum::{
    extract::{State, Path},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::shared::app_state::AppState;
use crate::core::{WorldId, UserId};
use uuid::Uuid;

pub async fn set_session_world(
    State(app): State<AppState>,
    Path(world_id): Path<WorldId>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // --- Extract session ---
    let Some(session_id) = headers
        .get("x-session-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
    else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    // --- Extract user ---
    let Some(user_id) = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
        .map(UserId::from_uuid)
    else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    // --- Activate world runtime ---
    if let Err(_) = app
        .world_engine
        .enter_world(user_id, world_id)
        .await
    {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    // --- Attach session to world ---
    if let Err(_) = app
        .session_source
        .set_world(session_id, world_id)
        .await
    {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    StatusCode::OK.into_response()
}

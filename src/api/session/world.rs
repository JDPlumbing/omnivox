use axum::{
    extract::{State, Json},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::shared::app_state::AppState;
use crate::core::WorldId;
use uuid::Uuid;
use crate::core::UserId;

#[derive(Deserialize)]
pub struct SetWorldReq {
    pub world_id: WorldId,
}

pub async fn set_session_world(
    State(app): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<SetWorldReq>,
) -> impl IntoResponse {
    let Some(session_id) = headers
        .get("x-session-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
    else {
        return StatusCode::UNAUTHORIZED.into_response();
    };


    let Some(user_id) = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
        .map(UserId::from_uuid)
    else {
        return StatusCode::UNAUTHORIZED.into_response();
    };


    let ownership = match app
        .ownership_source
        .resolve_ownership(user_id)
        .await
    {
        Ok(o) => o,
        Err(_) => return StatusCode::FORBIDDEN.into_response(),
    };

    // 🔒 Minimal rule for now:
    // if ownership.world_id exists, it must match
    if let Some(world_id) = ownership.world_id {
        if world_id != req.world_id {
            return StatusCode::FORBIDDEN.into_response();
        }
    }

    match app
        .session_source
        .set_world(session_id, req.world_id)
        .await
    {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

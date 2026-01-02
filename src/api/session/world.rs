use axum::{
    extract::State,
    http::HeaderMap,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::shared::app_state::AppState;


#[derive(Deserialize)]
pub struct SetWorldReq {
    pub world_id: i64,
}

pub async fn set_session_world(
    State(app): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<SetWorldReq>,
) -> impl IntoResponse {
    let Some(session_id) = headers
        .get("x-session-id")
        .and_then(|v| v.to_str().ok())
    else {
        return StatusCode::UNAUTHORIZED;
    };

    let _ = app
        .supa
        .from("anon_sessions")
        .update(json!({ "world_id": req.world_id }))
        .eq("session_id", session_id)
        .execute()
        .await;

    StatusCode::OK
}

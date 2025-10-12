use axum::{extract::State, response::IntoResponse, Json};
use axum::http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use crate::shared::app_state::AppState;

pub async fn init_session(
    State(app): State<AppState>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let maybe_session = headers
        .get("x-session-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok());

    if let Some(session_id) = maybe_session {
        let existing = app
            .supa
            .from("anon_sessions")
            .select("session_id, anon_owner_id")
            .eq("session_id", &session_id.to_string())
            .execute()
            .await;

        if let Ok(val) = existing {
            let rows: Vec<serde_json::Value> = serde_json::from_value(val.clone()).unwrap_or_default();
            if let Some(row) = rows.first() {
                return Json(json!({
                    "status": "ok",
                    "session_id": row["session_id"].as_str(),
                    "anon_owner_id": row["anon_owner_id"].as_str(),
                    "reused": true
                }))
                .into_response();
            }
        }
    }

    // Create new anon user + session
    let anon_owner_id = Uuid::new_v4();
    let session_id = Uuid::new_v4();

    let _ = app
        .supa
        .from("anon_users")
        .insert(json!({ "id": anon_owner_id }))
        .execute()
        .await;

    let _ = app
        .supa
        .from("anon_sessions")
        .insert(json!({
            "session_id": session_id,
            "anon_owner_id": anon_owner_id,
            "frame_id": 0,
        }))
        .execute()
        .await;

    Json(json!({
        "status": "ok",
        "session_id": session_id,
        "anon_owner_id": anon_owner_id,
        "reused": false
    }))
    .into_response()
}

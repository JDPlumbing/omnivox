use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;
use uuid::Uuid;

use crate::shared::app_state::AppState;
use crate::core::id::user_id::UserId;

pub async fn init_session(
    State(mut app): State<AppState>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {

    // ----------------------------
    // 1. Try to reuse existing session
    // ----------------------------
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
            let rows: Vec<serde_json::Value> =
                serde_json::from_value(val.clone()).unwrap_or_default();

            if let Some(row) = rows.first() {
                // NEW internal identity for this session
                let user_id = UserId::random();
                app.anon_owner_id = Some(user_id);

                return Json(json!({
                    "status": "ok",
                    "session_id": row["session_id"].as_str(),
                    "anon_owner_id": row["anon_owner_id"].as_str(),
                    "user_id": user_id.to_string(),
                    "reused": true
                }))
                .into_response();
            }
        }
    }

    // ----------------------------
    // 2. Create new anon user + session
    // ----------------------------
    let anon_owner_uuid = Uuid::new_v4();
    let session_uuid = Uuid::new_v4();

    // Internal engine identity
    let user_id = UserId::random();
    app.anon_owner_id = Some(user_id);

    // Store in Supabase
    let _ = app
        .supa
        .from("anon_users")
        .insert(json!({ "id": anon_owner_uuid }))
        .execute()
        .await;

    let _ = app
        .supa
        .from("anon_sessions")
        .insert(json!({
            "session_id": session_uuid,
            "anon_owner_id": anon_owner_uuid,
            "world_id": 0
        }))
        .execute()
        .await;

    // Return to frontend
    Json(json!({
        "status": "ok",
        "session_id": session_uuid.to_string(),
        "anon_owner_id": anon_owner_uuid.to_string(),
        "user_id": user_id.to_string(),
        "reused": false
    }))
    .into_response()
}

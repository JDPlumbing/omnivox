use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;
use uuid::Uuid;

use crate::shared::app_state::AppState;
use crate::core::id::user_id::UserId;

pub async fn init_session(
    State(app): State<AppState>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {

    // ----------------------------------------
    // 1️⃣ Try to reuse existing session
    // ----------------------------------------
    let maybe_session = headers
        .get("x-session-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok());

    if let Some(session_id) = maybe_session {
        if let Ok(row) = app
            .supa
            .from("anon_sessions")
            .select("session_id, engine_user_id")
            .eq("session_id", &session_id.to_string())
            .single()
            .await
        {
            // Reuse existing engine identity
            let engine_user_id = row["engine_user_id"]
                .as_str()
                .map(UserId::from_string)
                .unwrap_or_else(UserId::random);

            // Backfill if missing (older rows)
            if row["engine_user_id"].is_null() {
                let _ = app
                    .supa
                    .from("anon_sessions")
                    .update(json!({
                        "engine_user_id": engine_user_id.to_string()
                    }))
                    .eq("session_id", &session_id.to_string())
                    .execute()
                    .await;
            }

            return Json(json!({
                "status": "ok",
                "session_id": session_id.to_string(),
                "user_id": engine_user_id.to_string(),
                "reused": true
            }))
            .into_response();
        }
    }

    // ----------------------------------------
    // 2️⃣ Create new anon session
    // ----------------------------------------
    let session_uuid = Uuid::new_v4();
    let anon_owner_uuid = Uuid::new_v4();
    let engine_user_id = UserId::random();

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
            "engine_user_id": engine_user_id.to_string(),
            "world_id": 0
        }))
        .execute()
        .await;

    Json(json!({
        "status": "ok",
        "session_id": session_uuid.to_string(),
        "user_id": engine_user_id.to_string(),
        "reused": false
    }))
    .into_response()
}

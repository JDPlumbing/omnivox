use axum::{extract::State, http::HeaderMap, response::IntoResponse, Json};
use serde_json::json;
use crate::shared::app_state::AppState;
use crate::core::identity::user_map::map_or_create_user;

pub async fn session_status(
    State(app): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {

    // ----------------------------------------
    // 1️⃣ Authenticated user via JWT
    // ----------------------------------------
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_value) = auth_header.to_str() {
            let token = auth_value
                .strip_prefix("Bearer ")
                .unwrap_or(auth_value);

            if let Ok(sb_user) = app.supa.get_user_from_jwt(token.to_string()).await {
                if let Ok((user_id, role)) = map_or_create_user(&app, &sb_user).await {
                    // TEMP: default world until user-world mapping exists
                    let world_id = 1;

                    return Json(json!({
                        "session": {
                            "user_id": user_id.to_string(),
                            "role": role,
                            "is_anon": false,
                            "world_id": world_id
                        }
                    }))
                    .into_response();
                }
            }
        }
    }

    // ----------------------------------------
    // 2️⃣ Anonymous session via x-session-id
    // ----------------------------------------
    if let Some(sid) = headers
        .get("x-session-id")
        .and_then(|v| v.to_str().ok())
    {
        let row = app
            .supa
            .from("anon_sessions")
            .select("engine_user_id, world_id")
            .eq("session_id", sid)
            .single()
            .await;

        if let Ok(val) = row {
            let user_id = val.get("engine_user_id").and_then(|v| v.as_str());
            let world_id = val
                .get("world_id")
                .and_then(|v| v.as_i64())
                .unwrap_or(1);

            return Json(json!({
                "session": {
                    "user_id": user_id,
                    "role": "anon",
                    "is_anon": true,
                    "world_id": world_id
                }
            }))
            .into_response();
        }
    }

    // ----------------------------------------
    // 3️⃣ No session at all → anon default
    // ----------------------------------------
    Json(json!({
        "session": {
            "role": "anon",
            "is_anon": true,
            "world_id": 1
        }
    }))
    .into_response()
}

use axum::{extract::State, http::HeaderMap, response::IntoResponse, Json};
use serde_json::json;
use crate::shared::app_state::AppState;
use crate::core::identity::user_map::map_or_create_user;

pub async fn session_status(
    State(app): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {

    // 1️⃣ Authorization header present?
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_value) = auth_header.to_str() {
            let token = auth_value
                .strip_prefix("Bearer ")
                .unwrap_or(auth_value);

            // 2️⃣ Call Supabase
            match app.supa.get_user_from_jwt(token.to_string()).await {
                Ok(sb_user) => {
                    // 3️⃣ Map Supabase → internal user
                    match map_or_create_user(&app, &sb_user).await {
                        Ok((user_id, role)) => {
                            return Json(json!({
                                "session": {
                                    "user_id": user_id.to_string(),
                                    "role": role,
                                    "is_anon": false
                                }
                            }))
                            .into_response();
                        }
                        Err(err) => {
                            return Json(json!({
                                "session": {
                                    "user_id": app.anon_owner_id.map(|id| id.to_string()),
                                    "role": "anon",
                                    "is_anon": true
                                }
                            }))

                            .into_response();
                        }
                    }
                }

                Err(err) => {
                    // Supabase rejected JWT
                    return Json(json!({
                        "session": {
                            "role": "anon",
                            "is_anon": true,
                            "error": format!("invalid jwt: {}", err)
                        }
                    }))
                    .into_response();
                }
            }
        }
    }
// 2️⃣ Fallback to anon session via x-session-id
if let Some(sid) = headers
    .get("x-session-id")
    .and_then(|v| v.to_str().ok())
{
    let row = app
        .supa
        .from("anon_sessions")
        .select("engine_user_id")
        .eq("session_id", sid)
        .single()
        .await;

    if let Ok(val) = row {
        if let Some(uid) = val.get("engine_user_id").and_then(|v| v.as_str()) {
            return Json(json!({
                "session": {
                    "user_id": uid,
                    "role": "anon",
                    "is_anon": true
                }
            }))
            .into_response();
        }
    }
}

    // 4️⃣ No auth header → anon
    Json(json!({
        "session": {
            "role": "anon",
            "is_anon": true
        }
    }))
    .into_response()
}

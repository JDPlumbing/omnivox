use axum::{extract::State, http::HeaderMap, response::IntoResponse, Json};
use serde_json::json;
use crate::shared::app_state::AppState;

pub async fn verify_session(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Extract Bearer token
    let Some(auth_header) = headers.get("authorization") else {
        return Json(json!({ "valid": false, "error": "missing authorization header" })).into_response();
    };

    let auth_value = auth_header.to_str().unwrap_or_default();
    let token = auth_value.strip_prefix("Bearer ").unwrap_or(auth_value);

    // Try to verify using Supabase client
    let result = state.supa.get_user_from_jwt(token.to_string()).await;

    match result {
        Ok(user) => Json(json!({
            "valid": true,
            "user": user
        })).into_response(),
        Err(err) => Json(json!({
            "valid": false,
            "error": format!("invalid or expired token: {}", err)
        })).into_response(),
    }
}

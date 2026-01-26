use axum::{
    extract::State,
    http::HeaderMap,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use crate::shared::app_state::AppState;
use crate::supabasic::Supabase;

pub async fn verify_session(
    State(_state): State<AppState>, // unused for now, but keeps signature consistent
    headers: HeaderMap,
) -> impl IntoResponse {
    let Some(auth_header) = headers.get("authorization") else {
        return Json(json!({
            "valid": false,
            "error": "missing authorization header"
        })).into_response();
    };

    let auth_value = auth_header.to_str().unwrap_or_default();
    let token = auth_value.strip_prefix("Bearer ").unwrap_or(auth_value);

    let supa = match Supabase::new_from_env() {
        Ok(s) => s,
        Err(_) => {
            return Json(json!({
                "valid": false,
                "error": "auth backend unavailable"
            })).into_response();
        }
    };

    match supa.get_user_from_jwt(token.to_string()).await {
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

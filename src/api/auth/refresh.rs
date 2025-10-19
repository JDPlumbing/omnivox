use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;
use crate::shared::app_state::AppState;

pub async fn refresh_token(
    State(state): State<AppState>,
) -> impl IntoResponse {
    // 🔹 Call Supabase RPC for token refresh (assuming function exists)
    let response = state
        .supa
        .rpc("refresh_access_token", json!({})) // ✅ second arg added here
        .await;

    match response {
        Ok(val) => Json(json!({
            "status": "ok",
            "data": val
        })),
        Err(e) => Json(json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
}

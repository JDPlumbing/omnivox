use axum::{response::IntoResponse, Json};
use serde_json::json;
use crate::supabasic::Supabase;

pub async fn refresh_token() -> impl IntoResponse {
    let supa = match Supabase::new_from_env() {
        Ok(s) => s,
        Err(_) => {
            return Json(json!({
                "status": "error",
                "message": "auth backend unavailable"
            })).into_response();
        }
    };

    // 🔹 Call Supabase RPC for token refresh
    let response = supa
        .rpc("refresh_access_token", json!({}))
        .await;

    match response {
        Ok(val) => Json(json!({
            "status": "ok",
            "data": val
        })).into_response(),
        Err(e) => Json(json!({
            "status": "error",
            "message": e.to_string()
        })).into_response(),
    }
}

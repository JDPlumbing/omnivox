use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use crate::shared::app_state::AppState;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub user_id: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (axum::http::StatusCode, String)> {
    let client = Client::new();

    let supabase_url = std::env::var("SUPABASE_URL")
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Missing SUPABASE_URL".into()))?;
    let supabase_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY")
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Missing SUPABASE_SERVICE_ROLE_KEY".into()))?;

    let auth_url = format!("{}/auth/v1/token?grant_type=password", supabase_url);

    let res = client
        .post(&auth_url)
        .header("apikey", &supabase_key)
        .json(&serde_json::json!({
            "email": payload.email,
            "password": payload.password,
        }))
        .send()
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_GATEWAY, format!("Request failed: {}", e)))?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err((axum::http::StatusCode::UNAUTHORIZED, err_text));
    }

    let json: serde_json::Value = res.json().await
        .map_err(|_| (axum::http::StatusCode::BAD_GATEWAY, "Invalid JSON".into()))?;

    let access_token = json["access_token"]
        .as_str()
        .ok_or((axum::http::StatusCode::BAD_GATEWAY, "Missing token".into()))?
        .to_string();

    let user_id = json["user"]["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    Ok(Json(LoginResponse { access_token, user_id }))
}

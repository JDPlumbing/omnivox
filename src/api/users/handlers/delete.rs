use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;


use uuid::Uuid;
use reqwest::Client;
use std::env;
use crate::shared::app_state::AppState;
use crate::shared::identity::auth_context::{AuthContext, AccountRole};


use axum::extract::Extension;




//--------------------------
// DELETE USER
//------------------------
pub async fn delete_user(
    State(_app): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    let supabase_url =
        env::var("SUPABASE_URL").expect("SUPABASE_URL not set");
    let service_role_key =
        env::var("SUPABASE_SERVICE_ROLE_KEY").expect("SUPABASE_SERVICE_ROLE_KEY not set");

    let client = Client::new();

    let res = client
        .delete(format!(
            "{}/auth/v1/admin/users/{}",
            supabase_url, user_id
        ))
        .header("Authorization", format!("Bearer {}", service_role_key))
        .header("apikey", service_role_key)
        .send()
        .await;

    match res {
        Ok(r) if r.status().is_success() => {
            Json(json!({ "status": "deleted", "id": user_id })).into_response()
        }
        Ok(r) => {
            let text = r.text().await.unwrap_or_default();
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": text })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("{e:?}") })),
        )
            .into_response(),
    }
}
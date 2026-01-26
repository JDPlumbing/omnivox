use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use crate::shared::app_state::AppState;
use crate::api::users::payloads::create_user::CreateUserPayload;
use crate::infra::identity::supabase_user_admin::{self, NewUser};
// ------------------------------------------------------------
// POST /api/users/
// ------------------------------------------------------------
pub async fn create_user(
    State(_app): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    let new_user = NewUser {
        email: payload.email,
        password: payload.password,
        display_name: payload.display_name,
        role: payload.role,
    };

    match supabase_user_admin::provision_user(new_user).await {
        Ok(user) => Json(user).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

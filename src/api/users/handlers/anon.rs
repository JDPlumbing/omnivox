use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;
use serde_json::json;

use crate::shared::app_state::AppState;
use crate::shared::users::anon_user_source::AnonUserSource;

#[derive(serde::Deserialize)]
pub struct CreateAnonUserPayload {
    pub display_name: Option<String>,
}
// ------------------------------------------------------------
// GET /api/users/anon/{id}
// ------------------------------------------------------------
pub async fn get_anon_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.anon_user_source.get_anon_user(id).await {
        Ok(Some(user)) => Json(user).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// GET /api/users/anon
// ------------------------------------------------------------
pub async fn list_anon_users(
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.anon_user_source.list_anon_users().await {
        Ok(users) => Json(users).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// POST /api/users/anon
// ------------------------------------------------------------
pub async fn create_anon_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateAnonUserPayload>,
) -> impl IntoResponse {
    match state
        .anon_user_source
        .create_anon_user(payload.display_name)
        .await
    {
        Ok(user) => Json(user).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}


// ------------------------------------------------------------
// DELETE /api/users/anon/{id}
// ------------------------------------------------------------
pub async fn delete_anon_user(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "anon user deletion not implemented yet",
            "id": id
        })),
    )
}

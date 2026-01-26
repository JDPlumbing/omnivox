use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use serde_json::json;

use crate::shared::app_state::AppState;

// ------------------------------------------------------------
// GET /api/users/:id
// Admin-only user lookup (not implemented yet)
// ------------------------------------------------------------
pub async fn get_user(
    State(_app): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "admin user lookup not implemented yet",
            "user_id": user_id
        })),
    )
}

// ------------------------------------------------------------
// GET /api/users
// Admin-only user listing (not implemented yet)
// ------------------------------------------------------------
pub async fn list_users(
    State(_app): State<AppState>,
) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "admin user listing not implemented yet"
        })),
    )
}

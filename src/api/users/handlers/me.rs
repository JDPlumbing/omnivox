use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use crate::shared::app_state::AppState;
use crate::shared::identity::auth_context::{AuthContext, AccountRole};
use axum::extract::Extension;
// TODO(identity → ownership):
// Property + world context should be resolved via a PropertySource
// (and possibly a UserPropertySource) instead of inline DB queries.
// This is intentionally stubbed to preserve architecture.

//--------------------------
// GET ME
//--------------------------
pub async fn get_me(
    Extension(auth): Extension<AuthContext>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_id = auth.user_id;

    // Fetch basic user record
    let user = match state.user_source.get_user(user_id).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "user not found" })),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    };

    let account_role = match auth.role {
        AccountRole::Root => "root",
        AccountRole::User => "user",
    };

    Json(json!({
        "user": {
            "id": user.id.to_string(),
            "account_role": account_role
        },
        "property": null,
        "property_role": null,
        "world": null
    }))
    .into_response()
}

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

    let ownership = match state.ownership_source.resolve_ownership(user_id).await {
        Ok(o) => o,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    };
    let properties = match state
        .property_source
        .list_for_user(user_id)
        .await
    {
        Ok(p) => p,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    };

    Json(json!({
        "user": {
            "id": user.id.to_string(),
            "account_role": format!("{:?}", auth.role),
        },
        "ownership": {
            "property_id": ownership.property_id,
            "property_role": ownership.property_role,
            "world_id": ownership.world_id,
        },
        "properties": properties.iter().map(|p| json!({
            "property_id": p.property_id,
            "world_id": p.world_id,
        })).collect::<Vec<_>>(),
    }))
    .into_response()

}

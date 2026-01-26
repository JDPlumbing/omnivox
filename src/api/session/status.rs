use axum::{
    extract::Extension,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::shared::identity::auth_context::AuthContext;
use crate::shared::session::session_context::SessionContext;

pub async fn session_status(
    auth: Option<Extension<AuthContext>>,
    session: Option<Extension<SessionContext>>,
) -> impl IntoResponse {
    let auth = auth.map(|Extension(a)| a);
    let session = session.map(|Extension(s)| s);

    Json(json!({
        "session": {
            "user_id": auth.as_ref().map(|a| a.user_id.to_string()),
            "role": auth.as_ref().map(|a| format!("{:?}", a.role)),
            "is_anon": auth.is_none(),
            "world_id": session.as_ref().and_then(|s| s.world_id),
            "property_id": session.as_ref().and_then(|s| s.property_id),
        }
    }))
}

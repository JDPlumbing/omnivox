use axum::{Extension, Json};
use serde_json::json;
use crate::shared::identity::{
    request_context::RequestContext,
    auth_context::AuthContext,
};

pub async fn whoami(
    req_ctx: Option<Extension<RequestContext>>,
    auth: Option<Extension<AuthContext>>,
) -> Json<serde_json::Value> {
    match (req_ctx, auth) {
        (_, Some(Extension(auth))) => Json(json!({
            "authenticated": true,
            "user_id": auth.user_id,
            "role": auth.role,
        })),
        (Some(Extension(ctx)), None) => Json(json!({
            "authenticated": false,
            "is_anon": ctx.is_anon,
        })),
        _ => Json(json!({
            "authenticated": false,
            "is_anon": true,
        })),
    }
}

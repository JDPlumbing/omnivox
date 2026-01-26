use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::IntoResponse,
    Json,
};
use reqwest::StatusCode;

use crate::shared::identity::{
    auth_context::AuthContext,
    request_context::RequestContext,
    identity_source::IdentitySource,
};
use crate::infra::identity::supabase_identity_source::SupabaseIdentitySource;

pub async fn identity_middleware(
    mut req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    // --------------------------------------------------
    // Anonymous request
    // --------------------------------------------------
    let Some(token) = auth_header.and_then(|h| h.strip_prefix("Bearer ")) else {
        req.extensions_mut()
            .insert(RequestContext::anonymous(None));

        return next.run(req).await;
    };

    // --------------------------------------------------
    // Authenticated request
    // --------------------------------------------------
    let identity_source = match SupabaseIdentitySource::new_from_env() {
        Ok(src) => src,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Auth backend unavailable"
                })),
            )
                .into_response();
        }
    };

    let resolved = match identity_source.resolve_from_token(token).await {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Invalid token"
                })),
            )
                .into_response();
        }
    };


    // --------------------------------------------------
    // Attach request-scoped identity (CLEAN)
    // --------------------------------------------------
req.extensions_mut().insert(AuthContext {
    user_id: resolved.user_id,
    role: resolved.role,
});

req.extensions_mut()
    .insert(RequestContext::authenticated(None, resolved.user_id));


    next.run(req).await
}

use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::IntoResponse,
    Json,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::supabasic::Supabase;
use crate::shared::identity::{
    auth_context::{AuthContext, AccountRole},
    request_context::RequestContext,
};
use crate::core::UserId;

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
    let supa = match Supabase::new_from_env() {
        Ok(s) => s,
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

    let user = match supa.get_user_from_jwt(token.to_string()).await {
        Ok(u) => u,
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

    let supabase_user_id = Uuid::parse_str(
        user["id"]
            .as_str()
            .expect("Supabase user id missing"),
    )
    .expect("Invalid Supabase UUID");

    let user_id = UserId::from_uuid(supabase_user_id);

    let account_role = if std::env::var("ROOT_USER_ID")
        .map(|v| v == supabase_user_id.to_string())
        .unwrap_or(false)
    {
        AccountRole::Root
    } else {
        AccountRole::User
    };

    // --------------------------------------------------
    // Attach request-scoped identity
    // --------------------------------------------------
    req.extensions_mut().insert(AuthContext {
        supabase_user_id,
        user_id,
        account_role,
    });

    req.extensions_mut().insert(
        RequestContext::authenticated(None, user_id)
    );

    next.run(req).await
}

use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};

use crate::shared::app_state::AppState;
use crate::shared::identity::{
    auth_context::{AuthContext, AccountRole},
    request_context::RequestContext,
};

pub async fn identity_middleware(
    State(_app_state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    // 🔥 DEV MODE: always authenticated
    if cfg!(debug_assertions) {
        let user_id = crate::core::UserId::from_uuid(
            uuid::uuid!("00000000-0000-0000-0000-000000000001")
        );

        req.extensions_mut().insert(AuthContext {
            user_id,
            role: AccountRole::Root,
        });

        req.extensions_mut().insert(
            RequestContext::authenticated(None, user_id)
        );

        return next.run(req).await;
    }

    // prod fallback
    req.extensions_mut().insert(RequestContext::anonymous(None));
    next.run(req).await
}

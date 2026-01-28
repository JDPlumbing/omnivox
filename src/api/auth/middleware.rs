use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::shared::app_state::AppState;
use crate::shared::identity::request_context::RequestContext;

pub async fn identity_middleware(
    mut req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let state = req
        .extensions()
        .get::<AppState>()
        .expect("AppState missing from request");

    // ----------------------------------------
    // Extract session ID (optional)
    // ----------------------------------------
    let session_id = req
        .headers()
        .get("x-session-id")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok());

    // ----------------------------------------
    // No session → anonymous
    // ----------------------------------------
    let Some(session_id) = session_id else {
        req.extensions_mut()
            .insert(RequestContext::anonymous(None));
        return next.run(req).await;
    };

    // ----------------------------------------
    // Load session
    // ----------------------------------------
    let session = match state
        .session_source
        .get_session(session_id)
        .await
    {
        Ok(Some(s)) => s,
        _ => {
            // Invalid session → treat as anonymous
            req.extensions_mut()
                .insert(RequestContext::anonymous(Some(session_id)));
            return next.run(req).await;
        }
    };

    // ----------------------------------------
    // Authenticated vs anon
    // ----------------------------------------
    let ctx = if let Some(user_id) = session.user_id {
        RequestContext::authenticated(Some(session_id), user_id)
    } else {
        RequestContext::anonymous(Some(session_id))
    };

    req.extensions_mut().insert(ctx);

    next.run(req).await
}

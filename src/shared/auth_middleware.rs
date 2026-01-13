use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::{ Response, IntoResponse},
    Json,
    extract::State,
};
use uuid::Uuid;
use reqwest::StatusCode;

use crate::shared::app_state::AppState;
use crate::core::UserId;
use crate::shared::auth_context::AuthContext;



pub async fn populate_user_from_auth(
    State(app): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let Some(token) = auth_header.and_then(|h| h.strip_prefix("Bearer ")) else {
        return (
            StatusCode::UNAUTHORIZED,
            axum::Json(serde_json::json!({ "error": "Not authenticated" })),
        )
            .into_response();
    };

    let user = match app.supa.get_user_from_jwt(token.to_string()).await {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({ "error": "Invalid token" })),
            )
                .into_response();
        }
    };

    let uuid = uuid::Uuid::parse_str(user["id"].as_str().unwrap()).unwrap();

    req.extensions_mut().insert(AuthContext {
        supabase_user_id: uuid,
        user_id: UserId::from_uuid(uuid),
    });

    next.run(req).await
}

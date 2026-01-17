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
use crate::shared::auth_context::{ AuthContext, AccountRole };



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

    let supabase_user_id = Uuid::parse_str(
        user["id"]
            .as_str()
            .expect("Supabase user id missing"),
    )
    .expect("Invalid Supabase UUID");

    // derive your human-facing ID
    let user_id = UserId::from_uuid(supabase_user_id);

    // determine account role
        let account_role = if supabase_user_id.to_string()
            == std::env::var("ROOT_USER_ID").unwrap()
        {
            AccountRole::Root
        } else {
            AccountRole::User
        };


    // store resolved auth context
    req.extensions_mut().insert(AuthContext {
        supabase_user_id,
        user_id,
        account_role,
    });


    next.run(req).await
}

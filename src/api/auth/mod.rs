// src/api/auth/mod.rs
use axum::routing::{ post };
use axum::Router;
use crate::shared::app_state::AppState;

pub mod login;
pub mod verify;
pub mod refresh;
pub mod middleware;

    pub fn auth_routes() -> Router<AppState> {
        Router::new()
        .route("/login", post(login::login))
        .route("/verify", post(verify::verify_session))
        .route("/refresh", post(refresh::refresh_token))
    }
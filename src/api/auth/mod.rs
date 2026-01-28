// src/api/auth/mod.rs
use axum::routing::{ post, get };
use axum::Router;
use crate::shared::app_state::AppState;

pub mod signup;
pub mod login;
//pub mod verify;
//pub mod refresh;
pub mod middleware;
pub mod whoami;

    pub fn auth_routes() -> Router<AppState> {
        Router::new()
        .route("/signup", post(signup::signup))
        .route("/login", post(login::login))
        .route("/whoami", get(whoami::whoami))
        //.route("/verify", post(verify::verify_session))
        //.route("/refresh", post(refresh::refresh_token))
    }
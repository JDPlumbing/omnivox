// src/api/auth/mod.rs
pub mod login;
pub mod verify;
pub mod refresh;


    pub fn auth_routes() -> Router {
        Router::new()
        .route("/login", post(login))
        .route("/verify", post(verify_session))
        .route("/refresh", post(refresh_token))
    }
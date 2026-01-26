use axum::routing::{ get, post, put, patch, delete };
use axum::Router;
use crate::shared::app_state::AppState;

pub mod handlers;
pub mod dtos;
pub mod payloads;


    pub fn users_routes() -> Router<AppState> {
        Router::new()
            .route("/", get(handlers::admin::list_users))
            .route("/{id}", get(handlers::admin::get_user).delete(handlers::delete::delete_user))
            .route("/me", get(handlers::me::get_me))
            .route("/anon", get(handlers::anon::list_anon_users).post(handlers::anon::create_anon_user))
            .route("/anon/{id}", get(handlers::anon::get_anon_user).delete(handlers::anon::delete_anon_user))
        
    }
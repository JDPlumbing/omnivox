use axum::routing::{ get, post, put, patch, delete };
use axum::Router;

pub mod handlers;
pub mod create;

pub use handlers::*;
pub mod users;
pub use users::*;


    pub fn users_routes() -> Router {
        Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/me", get(get_me))
        .route("/{id}", get(get_user).delete(delete_user))
        .route("/anon", get(list_anon_users).post(create_anon_user))
        .route("/anon/{id}", get(get_anon_user))
        ;
    }
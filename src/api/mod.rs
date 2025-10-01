use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

mod worlds;
mod users;

pub use worlds::get_world;
pub use users::{get_user, get_anon_user, list_anon_users, create_anon_user};


async fn hello() -> &'static str {
    "Hello, World!"
}

/// Top-level API router
pub fn api_router() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/worlds/{id}", get(get_world))
        // real users (later, when hooked to auth)
        .route("/users/{id}", get(get_user))
        // anon users (for sim/testing)
        .route("/anon_users/{id}", get(get_anon_user))
        .route("/anon_users", get(list_anon_users).post(create_anon_user))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}

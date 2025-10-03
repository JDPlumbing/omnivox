use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

mod worlds;
mod users;
mod simulations;

// Worlds API handlers
pub use worlds::{get_world_handler, list_worlds_handler, create_world_handler};

// Users API handlers
pub use users::{get_user, get_anon_user, list_anon_users, create_anon_user};

// Simulations API handlers
pub use simulations::get_simulation;

/// Top-level API router
pub fn api_router() -> Router {
    Router::new()
        // Worlds
        .route("/worlds", get(list_worlds_handler).post(create_world_handler))
        .route("/worlds/{id}", get(get_world_handler))

        // Real users (later: tied to Supabase auth.users)
        .route("/users/{id}", get(get_user))

        // Anon users (for sim/testing)
        .route("/anon_users", get(list_anon_users).post(create_anon_user))
        .route("/anon_users/{id}", get(get_anon_user))

        // Simulations
        .route("/simulations/{id}", get(get_simulation))

        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}

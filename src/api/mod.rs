// src/api/mod.rs
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

// Worlds API
mod worlds;
pub use worlds::{list_worlds_handler, get_world_handler, create_world_handler};

// Simulations API
mod simulations;
pub use simulations::{list_simulations, get_simulation};

// Events API
mod events;
pub use events::{list_events_handler, get_event_handler, create_event_handler};

// Objex API
mod objex;
pub use objex::{list_objex, get_objex, create_objex_handler};

// Users API
mod users;
pub use users::{get_user, get_anon_user, list_anon_users, create_anon_user};

/// Top-level API router
pub fn api_router() -> Router {
    Router::new()
        // worlds
        .route("/worlds", get(list_worlds_handler).post(create_world_handler))
        .route("/worlds/{id}", get(get_world_handler))

        // simulations
        .route("/simulations", get(list_simulations))
        .route("/simulations/{id}", get(get_simulation))
        .route("/simulations/{id}/events", get(list_events_handler))

        // events
        .route("/events", post(create_event_handler))
        .route("/events/{id}", get(get_event_handler))

        // objex (scoped to simulation)
        .route("/objex", get(list_objex))
        .route("/objex/{id}", get(get_objex))
        .route("/simulations/{sim_id}/objex", post(create_objex_handler))

        // users
        .route("/users/{id}", get(get_user))
        .route("/anon_users", get(list_anon_users).post(create_anon_user))
        .route("/anon_users/{id}", get(get_anon_user))

        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}

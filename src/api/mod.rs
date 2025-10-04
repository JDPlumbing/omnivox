// src/api/mod.rs
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
// Users API
mod users;
pub use users::{get_user, get_anon_user, list_anon_users, create_anon_user};
// Worlds API
mod worlds;
pub use worlds::{list_worlds_handler, get_world_handler, create_world_handler};

// Simulations API
mod simulations;
pub use simulations::{list_simulations, get_simulation, seed_simulation};

// Objex and Events API
mod objex;
pub use objex::{create_objex, get_objex};
mod events;
pub use events::{create_event, list_events_for_sim};

/// Top-level API router
pub fn api_router() -> Router {
    Router::new()
        // tests
        .route("/ping", get(|| async { "pong" }))

        // users
        .route("/users/{id}", get(get_user))
        .route("/anon_users", get(list_anon_users).post(create_anon_user))
        .route("/anon_users/{id}", get(get_anon_user))

        // worlds
        .route("/worlds", get(list_worlds_handler).post(create_world_handler))
        .route("/worlds/{id}", get(get_world_handler))

        // simulations
        .route("/simulations", get(list_simulations))
        .route("/simulations/{id}", get(get_simulation))
        .route("/simulations/{id}/seed", post(seed_simulation))
        //.route("/simulations/{id}/events", get(list_events_handler))

        // objex
        .route("/objex", post(objex::create_objex))
        .route("/objex/{entity_id}", get(objex::get_objex))
        // events
        .route("/events/{entity_id}", post(events::create_event))
        .route("/events/{entity_id}", get(events::list_events_for_sim))
       

        


        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}

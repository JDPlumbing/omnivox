use axum::{
    routing::{get, post, put, patch, delete},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::shared::app_state::AppState;

// --- Users API ---
mod users;
pub use users::{get_user, get_anon_user, list_anon_users, create_anon_user};

// --- Worlds API ---
mod worlds;
pub use worlds::{list_worlds_handler, get_world_handler, create_world_handler, update_world_handler, patch_world_handler, delete_world_handler};

// --- Simulations API ---
mod simulations;
pub use simulations::{list_simulations, get_simulation, create_simulation, run_simulation};

// --- Entities API (NEW, replaces objex API) ---
mod entities;
pub use entities::{
    create_entity,
    get_entity,
    list_entities,
    list_entities_for_world,
    delete_entity,
};

// --- Events API ---
mod events;
pub use events::{
    create_event,
    list_events,
    get_event,
    update_event,
    patch_event,
    delete_event,
    list_events_for_sim,
    list_events_for_entity,
};

// --- Address API ---
mod address;
pub use address::*;

// --- Properties API ---
mod properties;
pub use properties::{
    list_properties,
    list_properties_for_world,
    get_property,
    create_property,
    update_property,
    delete_property,
};

// --- Session API ---
mod session;
pub use session::init_session;

// --- Pages API ---
mod pages;
pub use pages::{get_page, create_page, update_page, delete_page, list_pages};

// --- Auth API ---
mod auth;
use auth::{login::login, verify::verify_session, refresh::refresh_token};

pub fn api_router() -> Router<AppState> {
    // Users routes
    let users_routes = Router::new()
        .route("/{id}", get(get_user))
        .route("/anon", get(list_anon_users).post(create_anon_user))
        .route("/anon/{id}", get(get_anon_user));

    // Worlds routes
    let worlds_routes = Router::new()
        .route("/", get(list_worlds_handler).post(create_world_handler))
        .route("/{frame_id}",
            get(get_world_handler)
                .put(update_world_handler)
                .patch(patch_world_handler)
                .delete(delete_world_handler),
        );

    // Simulations routes
    let simulations_routes = simulations::routes();

    // Entities routes (NEW)
    let entities_routes = Router::new()
        .route("/", get(list_entities).post(create_entity))
        .route("/world/{world_id}", get(list_entities_for_world))
        .route(
            "/{entity_id}",
            get(get_entity)
                .delete(delete_entity),
        );

    // Events routes
    let events_routes = Router::new()
        .route("/", get(list_events).post(create_event))
        .route("/sim/{simulation_id}", get(list_events_for_sim))
        .route("/entity/{entity_id}", get(list_events_for_entity))
        .route(
            "/{event_id}",
            get(get_event)
                .put(update_event)
                .patch(patch_event)
                .delete(delete_event),
        );

    // Address routes
    let address_routes = Router::new()
        .route("/", get(list_addresses).post(create_address))
        .route(
            "/{id}",
            get(get_address)
                .put(update_address)
                .patch(patch_address)
                .delete(delete_address),
        )
        .route("/{id}/resolve", post(resolve_address));

    // Properties routes
    let property_routes = Router::new()
        .route("/", get(list_properties).post(create_property))
        .route(
            "/{id}",
            get(get_property)
                .put(update_property)
                .delete(delete_property),
        )
        .route("/world/{world_id}", get(list_properties_for_world));

    // Pages routes
    let pages_routes = Router::new()
        .route("/", get(list_pages).post(create_page))
        .route("/{slug}", get(get_page))
        .route("/id/{id}", put(update_page))
        .route("/{slug}", delete(delete_page));

    // Auth routes
    let auth_routes = Router::new()
        .route("/login", post(login))
        .route("/verify", post(verify_session))
        .route("/refresh", post(refresh_token));

    Router::new()
        .route("/ping", get(|| async { "pong" }))
        .nest("/auth", auth_routes)
        .route("/session/init", get(init_session))
        .nest("/address", address_routes)
        .nest("/properties", property_routes)
        .nest("/users", users_routes)
        .nest("/worlds", worlds_routes)
        .nest("/simulations", simulations_routes)
        .nest("/entities", entities_routes)
        .nest("/events", events_routes)
        .nest("/pages", pages_routes)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}

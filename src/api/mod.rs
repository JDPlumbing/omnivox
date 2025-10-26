use axum::{
    routing::{get, post, put, patch, delete},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

// --- Users API ---
mod users;
pub use users::{get_user, get_anon_user, list_anon_users, create_anon_user};
// --- Worlds API ---
mod worlds;
pub use worlds::{list_worlds_handler, get_world_handler, create_world_handler};
// --- Simulations API ---
mod simulations;
pub use simulations::{list_simulations, get_simulation, run_simulation, create_simulation};
// --- Objex + Events API ---
mod objex;
pub use objex::{create_objex, get_objex};
mod events;
pub use events::{create_event, list_events_for_sim, list_events_for_entity};
// --- Address API ---
mod address;
pub use address::*;
// --- Properties API ---
mod properties;
pub use properties::{get_property, list_properties, create_property, delete_property};
// --- Main API ---
use crate::shared::app_state::AppState;
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
        .route("/", get(worlds::list_worlds_handler).post(worlds::create_world_handler))
        .route("/{frame_id}", 
            get(worlds::get_world_handler)
            .put(worlds::update_world_handler)
            .patch(worlds::patch_world_handler)
            .delete(worlds::delete_world_handler)
        );

    // Simulations routes
    let simulations_routes = Router::new()
        .route("/", get(simulations::list_simulations).post(simulations::create_simulation))
        .route("/{id}", get(simulations::get_simulation)
            .put(simulations::update_simulation)
            .patch(simulations::patch_simulation)
            .delete(simulations::delete_simulation))
        .route("/init", post(simulations::init_simulation))
        .route("/run", post(simulations::run_simulation));

    // Objex routes — order matters!
    let objex_routes = Router::new()
        .route("/", get(objex::list_objex).post(objex::create_objex))
        .route("/world/{frame_id}", get(objex::list_objex_for_world))
        .route(
            "/{entity_id}",
            get(objex::get_objex)
                .put(objex::update_objex)
                .patch(objex::patch_objex)
                .delete(objex::delete_objex),)
        .route("/property/{property_id}", get(objex::list_objex_for_property)

        );

    // Events routes
    let events_routes = Router::new()
        .route("/", get(events::list_events).post(events::create_event))
        .route("/sim/{simulation_id}", get(events::list_events_for_sim))
        .route("/entity/{entity_id}", get(events::list_events_for_entity))
        .route("/{event_id}",
            get(events::get_event)
                .put(events::update_event)
                .patch(events::patch_event)
                .delete(events::delete_event),
        );
    
    let address_routes = Router::new()
        .route("/", get(list_addresses).post(create_address))
        .route("/{id}", get(get_address)
            .put(update_address)
            .patch(patch_address)
            .delete(delete_address))
        .route("/{id}/resolve", post(resolve_address));

    let property_routes = Router::new()
        .route("/", get(properties::list_properties).post(properties::create_property))
        .route(
            "/{id}",
            get(properties::get_property)
                .put(properties::update_property)
                .delete(properties::delete_property),
        )
        .route(
            "/world/{frame_id}",
            get(properties::list_properties_for_world),
        )
        .route("/{id}/generate", post(properties::generate_property_objects));

    // Pages routes
    let pages_routes = Router::new()
        .route("/", get(list_pages).post(create_page))          // Create
        .route("/{slug}", get(get_page))         // Read
        .route("/id/{id}", put(update_page))     // Update
        .route("/{slug}", delete(delete_page));  // Delete
        
    let auth_routes = Router::new()
        .route("/login", post(login))
        .route("/verify", post(verify_session)) 
        .route("/refresh", post(refresh_token));

    Router::new()
        // --- Test route ---
        .route("/ping", get(|| async { "pong" }))

        // --- Auth routes ---
        .nest("/auth", auth_routes)
        .route("/session/init", get(init_session))

        // --- Main API routes ---
        .nest("/address", address_routes)
        .nest("/properties", property_routes)
        
        .nest("/users", users_routes)
        .nest("/worlds", worlds_routes)
        .nest("/simulations", simulations_routes)
        .nest("/objex", objex_routes)
        .nest("/events", events_routes)
        .nest("/pages", pages_routes) 
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}

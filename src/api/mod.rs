use axum::{
    routing::{get, post, put, patch, delete},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

use crate::shared::app_state::AppState;
use axum::middleware;
use crate::shared::auth_middleware::populate_user_from_auth;

// --- Time API ---
mod time;
pub use time::{time_routes};

// --- Users API ---
mod users;
pub use users::{get_user, 
                get_anon_user, 
                list_anon_users, 
                create_anon_user, 
                list_users, 
                create_user,
                delete_user,
                get_me,
            };

// --- Worlds API ---
mod worlds;
pub use worlds::*;

// --- Simulations API ---
//mod simulations;
//pub use simulations::{list_simulations, get_simulation, create_simulation};

// --- Entities API (NEW, replaces objex API) ---
mod entities;
pub use entities::{
    //create_entity,
    get_entity,
    list_entities,
    list_entities_for_world,
    delete_entity,
    create_entities,
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
mod location;
pub use location::*;

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
pub use session::{init_session, session_status, set_session_world};

// --- Pages API ---
mod pages;
pub use pages::*;

// --- Auth API ---
mod auth;
use auth::{login::login, verify::verify_session, refresh::refresh_token};

/*
mod viewer;
use viewer::viewer_routes;
*/
mod objex;
use objex::objex::objex_routes;
use objex::materials::material_routes;
use objex::geospec::geospec_routes;
use objex::templates::geometry_template_routes;

// -------- physics API -----
mod physics;
use physics::*;


// -------observers API 
mod observers;
use observers::*;

pub fn api_router(app_state: AppState) -> Router {
    // Users routes
    let users_routes = Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/me", get(get_me))
        .route("/{id}", get(get_user).delete(delete_user))
        .route("/anon", get(list_anon_users).post(create_anon_user))
        .route("/anon/{id}", get(get_anon_user))
        ;

    let worlds_routes = worlds::world_routes();

    let physics_routes = Router::new()
        .route("/lunar/phase", get(lunar_phase_handler))
        .route("/illumination", get(solar_illumination_handler))
        .route("/seasons/check", get(seasons_check_handler))
        .route("/tides", get(tidal_potential_handler))
        .route("/tides/acceleration", get(tidal_acceleration_handler))
        .route("/tides/curve", get(tides_curve_handler))
        .route("/insolation/daily", get(daily_insolation_handler))
        .route("/insolation/curve", get(insolation_curve_handler))
        .route("/insolation/seasons", get(insolation_seasons_handler))
        .route("/environmental_snapshot", get(environmental_snapshot_handler))
        .route("/environmental_snapshot/curve", get(environmental_snapshot_curve_handler))
        
        
        
        ;


    //let simulations_routes = simulations::routes();

    let entities_routes = Router::new()
        .route("/", get(list_entities).post(create_entities))
        .route("/world/{world_id}", get(list_entities_for_world))
        .route(
            "/{entity_id}",
            get(get_entity).delete(delete_entity),
        );

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
    
    let location_routes = location::location_routes();

    let property_routes = Router::new()
        .route("/", get(list_properties).post(create_property))
        .route(
            "/{id}",
            get(get_property)
                .put(update_property)
                .delete(delete_property),
        )
        .route("/world/{world_id}", get(list_properties_for_world));

    let pages_routes = pages_routes();

    let auth_routes = Router::new()
        .route("/login", post(login))
        .route("/verify", post(verify_session))
        .route("/refresh", post(refresh_token));

    let time_routes = time::time_routes();
    //let viewer_routes = viewer::viewer_routes();
    let observer_routes = observers::observer_routes();

    Router::new()
        .route("/ping", get(|| async { "pong" }))
        .nest("/auth", auth_routes)
        .route("/session/init", get(init_session))
        .route("/session/status", get(session_status))
        .route("/session/world", post(set_session_world))


        .nest("/location", location_routes)
        .nest("/properties", property_routes)
        .nest(
            "/users",
            users_routes.layer(
                middleware::from_fn_with_state(app_state.clone(), populate_user_from_auth),
            ),
        )


        .nest("/worlds", worlds_routes)
        //.nest("/simulations", simulations_routes)
        .nest("/entities", entities_routes)
        .nest("/objex", objex_routes())
        .nest("/objex/materials", material_routes())
        .nest("/objex/geospec", geospec_routes())
        .nest("/physics", physics_routes)
        
        .nest("/geometry/templates", geometry_template_routes())
        .nest("/events", events_routes)
        .nest("/time", time_routes)
        .nest("/pages", pages_routes)
        //.nest("/viewer", viewer_routes)
        .nest("/observers", observer_routes)

        .with_state(app_state)
        
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
                .allow_headers(Any)
                .expose_headers(Any)
        )

}


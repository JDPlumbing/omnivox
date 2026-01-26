use axum::{
    routing::{get, post, put, patch, delete},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

use crate::shared::app_state::AppState;
use axum::middleware;
use crate::api::auth::middleware::identity_middleware;
// --- Auth API ---
pub mod auth;

// --- Time API ---
pub mod time;
pub use time::{time_routes};
// --- Worlds API ---
pub mod worlds;
pub use worlds::{world_routes};

// --- Users API ---
pub mod users;
pub use users::users_routes;


// --- Simulations API ---
//mod simulations;
//pub use simulations::{list_simulations, get_simulation, create_simulation};

// --- Entities API (NEW, replaces objex API) ---
//mod entities;
//pub use entities::*;


// --- Events API ---
//mod events;


// --- Address API ---
//mod location;


// --- Properties API ---
//mod properties;
// --- Session API ---
//mod session;


// --- Pages API ---
//mod pages;
//pub use pages::*;



/*
mod viewer;


mod objex;
use objex::objex::objex_routes;
use objex::materials::material_routes;
use objex::geospec::geospec_routes;
use objex::templates::geometry_template_routes;
*/
// -------- physics API -----
//mod physics;
//use physics::*;


// -------observers API 
//mod observers;
//use observers::*;

pub fn api_router(app_state: AppState) -> Router {
    // Auth routes
    let auth_routes = auth::auth_routes();
    // Users routes
    let users_routes = users::users_routes();
    // Time routes
    let time_routes = time::time_routes();
    // Worlds routes
    let worlds_routes = worlds::world_routes();



    //

    //let physics_routes = physics::physics_routes();


    //let simulations_routes = simulations::routes();

    //let entities_routes = entities::entities_routes();

    //let events_routes = events::events_routes();

//    let location_routes = location::location_routes();

  //  let property_routes = properties::property_routes();

    //let pages_routes = pages::pages_routes();

    //

  
    //let viewer_routes = viewer::viewer_routes();
    //let observer_routes = observers::observer_routes();

    Router::new()
        .route("/ping", get(|| async { "pong" }))
        .nest("/auth", auth_routes)
        .nest("/time", time_routes)
        .nest("/worlds", worlds_routes)
        .nest("/users", users_routes)
        //.route("/session/init", get(init_session))
        //.route("/session/status", get(session_status))
        //.route("/session/world", post(set_session_world))


        //.nest("/location", location_routes)
        //.nest("/properties", property_routes)

        
        //.nest("/simulations", simulations_routes)
        //.nest("/entities", entities_routes)
        //.nest("/objex", objex_routes())
        //.nest("/objex/materials", material_routes())
        //.nest("/objex/geospec", geospec_routes())
       // .nest("/physics", physics_routes)
        
        //.nest("/geometry/templates", geometry_template_routes())
        //.nest("/events", events_routes)
        
        //.nest("/pages", pages_routes)
        //.nest("/viewer", viewer_routes)
      // .nest("/observers", observer_routes)

        .with_state(app_state)
        
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
                .allow_headers(Any)
                .expose_headers(Any)
        )

}


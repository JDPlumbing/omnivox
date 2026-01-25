use axum::{
    routing::{get, post, put, patch, delete},
    Router,
};

pub mod events;
pub use events::*;


pub fn events_routes() -> Router {
    Router::new()
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
}
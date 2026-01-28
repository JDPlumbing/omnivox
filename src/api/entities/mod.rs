use axum::Router;
use axum::routing::{post, get, put, delete};
use crate::shared::app_state::AppState;

pub mod handlers;
pub mod dtos;
pub mod payloads;

pub fn entities_routes() -> Router<AppState> {
    Router::new()
        .route("/time", post(handlers::create_time::create_time_entity))
        .route("/note", post(handlers::create_note::create_note_entity))
        .route("/{id}", get(handlers::get_entity::get_entity))
        .route("/{id}/world", put(handlers::set_world::set_entity_world))
    	.route("/{id}/position", put(handlers::set_position::set_entity_position))
	    .route("/{id}/spawned_at", put(handlers::set_spawned_at::set_entity_spawned_at))
.route(
    "/{id}/despawned_at",
    put(handlers::set_despawned_at::set_entity_despawned_at),
)





}

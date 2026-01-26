use axum::{Router, routing::{get, post, put, delete}};
use crate::shared::app_state::AppState;

pub mod handlers;
pub mod dtos;
pub mod payloads;

pub fn property_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list::list_properties))
        .route("/{id}", get(handlers::get::get_property))
        .route("/", post(handlers::create::create_property))
        .route("/{id}", put(handlers::update::update_property))
        .route("/{id}", delete(handlers::delete::delete_property))
}

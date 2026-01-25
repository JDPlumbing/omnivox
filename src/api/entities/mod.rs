use axum::Router;
use axum::routing::{get, post};
use crate::shared::app_state::AppState; 
pub mod entities;
pub use entities::*;
pub mod runtime;




pub fn entities_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_entities).post(create_entities))
        .route("/world/{world_id}", get(list_entities_for_world))
        .route(
            "/{entity_id}",
            get(get_entity).delete(delete_entity),
        )
        .route("/spawn_with_position/world/{world_id}", post(runtime::spawn_entity_with_position))

    }
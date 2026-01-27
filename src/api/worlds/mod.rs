use axum::{ Router};
use axum::routing::{ get, post };
use crate::shared::AppState;

pub mod dtos;

pub mod handlers;

pub mod payloads;



pub fn world_routes() -> Router<AppState> {
    Router::new()
                .route("/", get(handlers::list::list_worlds_handler).post(handlers::create::create_world_handler))
        .route(
            "/{world_id}",
            get(handlers::get::get_world_handler)
                .put(handlers::update::update_world_handler)
                //.patch(handlers::patch::patch_world_handler)
                .delete(handlers::delete::delete_world_handler),
        )
        .route("/{world_id}/stats", get(handlers::stats::get_world_stats_handler))
        .route("/{world_id}/time/now", get(handlers::time_now::world_time_now_handler))
        .route("/{world_id}/epoch/set", post(handlers::set_epoch::set_world_epoch_handler))
        .route("/{world_id}/environment/sample", get(handlers::environment::sample_environment_handler))
        .route("/{from}/relative/{to}", get(handlers::relative::world_relative_handler))
        .route("/{from}/relative/{to}/origin", get(handlers::relative::world_origin_relative_handler))
        .route("/{world_id}/properties", get(handlers::properties::list_world_properties))
        .route("/{world_id}/entities", get(handlers::list_entities::list_entities_in_world))
}

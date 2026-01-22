use axum::{ Router};
use axum::routing::{ get, post };
use crate::shared::AppState;



pub mod time_now;
pub use time_now::world_time_now;

pub mod worlds;
pub use worlds::*;

pub mod set_epoch;
pub use set_epoch::*;

pub mod dto;
pub use dto::*;

pub mod handlers;
pub use handlers::*;






pub fn world_routes() -> Router<AppState> {
    Router::new()
                .route("/", get(list_worlds_handler).post(create_world_handler))
        .route(
            "/{world_id}",
            get(get_world_handler)
                .put(update_world_handler)
                .patch(patch_world_handler)
                .delete(delete_world_handler),
        )
        .route("/{world_id}/stats", get(get_world_stats))
        .route("/{world_id}/time/now", get(world_time_now))
        .route("/{world_id}/epoch/set", post(set_world_epoch))
        .route("/{world_id}/environment/sample", get(sample_environment_handler))
        .route("/{from}/relative/{to}", get(world_relative_handler))
        .route("/{from}/relative/{to}/origin", get(world_origin_relative_handler))


}

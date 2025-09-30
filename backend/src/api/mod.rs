use axum::Router;
use axum::routing::get;

mod worlds;
use worlds::get_world;

pub fn api_router() -> Router {
    Router::new()
        .route("/worlds/{id}", get(get_world))
}

use axum::Router;
use axum::routing::get;

mod worlds;
use worlds::get_world;


pub fn hello() -> &'static str {
    "Hello, World!"
}

pub fn api_router() -> Router {
    Router::new()
        .route("/worlds/{id}", get(get_world))
        .route("/hello", get(hello))
}

use axum::{Router, routing::{get, post}};
use crate::shared::app_state::AppState;

mod camera;
pub use camera::*;

pub fn viewer_routes() -> Router<AppState> {
    Router::new()
        .route("/camera/delta", post(post_camera_delta))
        .route("/camera", get(get_camera_state))
}

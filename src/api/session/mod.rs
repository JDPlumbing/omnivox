use axum::{routing::{get, post}, Router};
use crate::shared::app_state::AppState;

pub mod init;
pub mod status;
//pub mod world;

pub use init::init_session;
pub use status::session_status;
//pub use world::set_session_world;

pub fn session_routes() -> Router<AppState> {
    Router::new()
        .route("/init", post(init_session))
        .route("/status", get(session_status))
        //.route("/world/{world_id}", post(set_session_world))
}
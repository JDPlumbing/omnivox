use axum::{routing::{get, post}, Router};
use crate::shared::app_state::AppState;

pub mod init;
pub mod status;
pub mod world;

pub use init::init_session;
pub use status::session_status;
pub use world::set_session_world;

pub fn session_routes() -> Router<AppState> {
    Router::new()
        .route("/session/init", get(init_session))
        .route("/session/status", get(session_status))
        .route("/session/world", post(set_session_world))
}
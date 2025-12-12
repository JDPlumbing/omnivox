mod now;
mod format;
mod duration;
mod delta;
mod simdate;
mod julian;
use crate::shared::AppState; //
use axum::routing::{get, post};
use axum::Router;

pub fn time_routes() -> Router<AppState> {
    Router::new()
        .route("/simtime/now", get(now::simtime_now))
        .route("/format", get(format::format_simtime_handler))
        .route("/duration/human", get(duration::duration_human_handler))
        .route("/delta", post(delta::delta_handler))
        .route("/simdate/from_ns", get(simdate::simdate_from_ns_handler))
        .route("/julian/from_ns", get(julian::julian_from_ns_handler))
}

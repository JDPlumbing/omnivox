use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_display::format_simdate;

// GET /api/worlds/{world_id}/time/now
pub async fn world_time_now_handler(
    State(app): State<AppState>,
    Path(world_id): Path<WorldId>,
) -> impl IntoResponse {
    // --------------------------------------------------
    // 1. Load world metadata via WorldSource
    // --------------------------------------------------

    let world = match app.world_source.get_world(world_id).await {
        Ok(w) => w,
        Err(e) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": "world not found",
                    "details": e.to_string()
                })),
            )
                .into_response();
        }
    };

    // --------------------------------------------------
    // 2. Global absolute simtime
    // --------------------------------------------------

    let sim_now = SimTime::now();

    // --------------------------------------------------
    // 3. World epoch (stored as Option<String>)
    // --------------------------------------------------

    let epoch = world
        .world_epoch
        .as_deref()
        .and_then(|s| s.parse::<i128>().ok())
        .map(SimTime::from_ns)
        .unwrap_or_else(|| SimTime::from_ns(0));

    // --------------------------------------------------
    // 4. World-local simtime (new origin, same unit)
    // --------------------------------------------------

    let world_time = SimTime::from_ns(
        sim_now.as_ns() - epoch.as_ns()
    );

    // --------------------------------------------------
    // 5. Deterministic calendar formatting
    // --------------------------------------------------

    let world_date = format_simdate(world_time);

    Json(json!({
        "world_id": world_id,

        // absolute global simtime
        "simtime_ns": sim_now.as_ns().to_string(),

        // epoch for this world
        "world_epoch_ns": epoch.as_ns().to_string(),

        // world-local simtime
        "world_time_ns": world_time.as_ns().to_string(),

        // formatted synthetic date
        "world_date": world_date
    }))
    .into_response()
}

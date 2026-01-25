use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_date::SimDate;

#[derive(Debug, Deserialize)]
pub struct SetEpochRequest {
    /// Target world-local date (YYYY-MM-DD)
    pub target_simdate: String,
}

pub async fn set_world_epoch_handler(
    State(app): State<AppState>,
    Path(world_id): Path<WorldId>,
    Json(payload): Json<SetEpochRequest>,
) -> impl IntoResponse {
    // --------------------------------------------------
    // 1. Parse YYYY-MM-DD
    // --------------------------------------------------

    let parts: Vec<&str> = payload.target_simdate.split('-').collect();
    if parts.len() != 3 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Invalid date format" })),
        )
            .into_response();
    }

    let date = match (
        parts[0].parse::<i32>(),
        parts[1].parse::<u8>(),
        parts[2].parse::<u8>(),
    ) {
        (Ok(y), Ok(m), Ok(d)) => SimDate { year: y, month: m, day: d },
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Invalid date values" })),
            )
                .into_response();
        }
    };

    // --------------------------------------------------
    // 2. Compute epoch
    // --------------------------------------------------

    let target = date.to_sim_time();
    let now = SimTime::now();
    let new_epoch = SimTime::from_ns(
        now.as_ns() - target.as_ns()
    );

    // --------------------------------------------------
    // 3. Persist via WorldSource
    // --------------------------------------------------

    if let Err(e) = app
        .world_source
        .set_world_epoch(world_id, new_epoch)
        .await
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "failed to update epoch",
                "details": e.to_string()
            })),
        )
            .into_response();
    }

    // --------------------------------------------------
    // 4. Invalidate runtime cache
    // --------------------------------------------------

    app.worlds.write().await.remove(&world_id);

    // --------------------------------------------------
    // 5. Response
    // --------------------------------------------------

    Json(serde_json::json!({
        "status": "ok",
        "world_id": world_id,
        "new_epoch_ns": new_epoch.as_ns().to_string(),
        "target_simdate": payload.target_simdate
    }))
    .into_response()
}

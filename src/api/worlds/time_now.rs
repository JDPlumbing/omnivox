use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;

use crate::shared::app_state::AppState;
use crate::supabasic::worlds::WorldRow;
use crate::core::id::WorldId;

use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_display::format_simdate;

// GET /api/worlds/{world_id}/time/now
pub async fn world_time_now(
    State(state): State<AppState>,
    Path(raw_id): Path<i64>,
) -> Json<serde_json::Value> {

    // Convert i64 → WorldId
    let world_id = WorldId(raw_id);

    // Load world metadata
    let world = WorldRow::get(&state.supa, world_id)
        .await
        .expect("Failed to load world");

    // Global absolute simtime
    let sim_now = SimTime::now();

    // World epoch (stored as Option<i128> nanoseconds)
    let epoch = world
        .world_epoch
        .as_deref()
        .and_then(|s| s.parse::<i128>().ok())
        .map(SimTime::from_ns)
        .unwrap_or_else(|| SimTime::from_ns(0));

    // Compute world-local time as a SimTime,
    // NOT as a SimDuration (important!)
    let world_time = SimTime::from_ns(sim_now.as_ns() - epoch.as_ns());

    // Format into synthetic simdate
    let world_date = format_simdate(world_time);

    Json(json!({
        "world_id": raw_id,

        // absolute global sim time
        "simtime_ns": sim_now.as_ns().to_string(),

        // epoch for this world
        "world_epoch_ns": epoch.as_ns().to_string(),

        // world-local simtime (same unit, new origin)
        "world_time_ns": world_time.as_ns().to_string(),

        // formatted date using your deterministic calendar
        "world_date": world_date
    }))
}

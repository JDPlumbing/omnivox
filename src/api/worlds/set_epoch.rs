use axum::{Json, extract::{Path, State}};
use serde::Deserialize;
use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_date::SimDate;
use crate::shared::AppState;

#[derive(Debug, Deserialize)]
pub struct SetEpochRequest {
    pub target_simdate: String, // "YYYY-MM-DD"
}

pub async fn set_world_epoch(
    State(state): State<AppState>,
    Path(world_id): Path<i64>,
    Json(payload): Json<SetEpochRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {

    // ----- 1. Parse YYYY-MM-DD -----
    let parts: Vec<&str> = payload.target_simdate.split('-').collect();
    if parts.len() != 3 {
        return Err((axum::http::StatusCode::BAD_REQUEST, "Invalid date".into()));
    }

    let date = SimDate {
        year:  parts[0].parse().unwrap(),
        month: parts[1].parse().unwrap(),
        day:   parts[2].parse().unwrap(),
    };

    // ----- 2. Compute Epoch -----
    let target = date.to_sim_time();
    let now = SimTime::now();
    let new_epoch_ns = now.as_ns() - target.as_ns();

    // ----- 3. Supabasic update -----
    let supa = &state.supa;

    supa.from("worlds")
        .update(serde_json::json!({
            "world_epoch": new_epoch_ns.to_string()
        }))
        .eq("world_id", &world_id.to_string())
        .execute()
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // ----- 4. Response -----
    Ok(Json(serde_json::json!({
        "status": "ok",
        "world_id": world_id,
        "new_epoch_ns": new_epoch_ns.to_string(),
        "target_simdate": payload.target_simdate
    })))
}

use axum::{
    extract::{State, Path},
    response::IntoResponse,
    Json
};
use serde_json::{json, Value};

use crate::shared::app_state::AppState;
use crate::core::id::simulation_id::SimulationId;
use crate::sim::simulations::simulation_config::SimulationConfig;
use crate::core::id::{WorldId, UvoxRegionId, UserId};
use axum::http::StatusCode;

//
// ─────────────────────────────────────────────────────────
// 1. FIX: Parse SimulationId from String instead of Path<T>
// ─────────────────────────────────────────────────────────
//

// Custom helper
fn parse_sim_id(id_str: &str) -> Result<SimulationId, String> {
    id_str.parse::<SimulationId>()
        .map_err(|_| format!("Invalid SimulationId '{}'", id_str))
}

//
// ───────────────────────────────────────────────
//   TICK SIM
//   POST /api/simulations/runtime/{id}/tick
// ───────────────────────────────────────────────
//
#[axum::debug_handler]
pub async fn tick_sim(
    State(app): State<AppState>,
    Path(id_str): Path<String>,            // FIXED
) -> impl IntoResponse {

    let sim_id = match parse_sim_id(&id_str) {
        Ok(v) => v,
        Err(e) => return Json(json!({ "status": "error", "message": e })),
    };

    let mut mgr = app.sim_manager.write().await;

    let events = mgr.tick(sim_id).await.unwrap_or_default();

    Json(json!({
        "status": "ticked",
        "simulation_id": sim_id.to_string(), // FIXED
        "event_count": events.len(),
        "events": events
    }))
}

//
// ───────────────────────────────────────────────
//   START SIM
//   POST /api/simulations/runtime/start
// ───────────────────────────────────────────────
//
#[axum::debug_handler]
pub async fn start_sim(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let cfg = SimulationConfig::basic(
        WorldId(0),
        UvoxRegionId::default(),
        UserId(0),
    );

    let mut mgr = state.sim_manager.write().await;

    match mgr.start(cfg).await {
        Ok(sim_id) => (
            StatusCode::OK,
            Json(json!({ "simulation_id": sim_id.to_string() })),
        ),

        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": err.to_string() })),
        ),
    }
}

//
// ───────────────────────────────────────────────
//   STOP SIM
//   POST /api/simulations/runtime/{id}/stop
// ───────────────────────────────────────────────
//
#[axum::debug_handler]
pub async fn stop_sim(
    State(app): State<AppState>,
    Path(id_str): Path<String>,            // FIXED
) -> impl IntoResponse {

    let sim_id = match parse_sim_id(&id_str) {
        Ok(v) => v,
        Err(e) => return Json(json!({ "status": "error", "message": e })),
    };

    let mut mgr = app.sim_manager.write().await;

    match mgr.stop(sim_id).await {
        Ok(true) => Json(json!({
            "status": "stopped",
            "simulation_id": sim_id.to_string()
        })),
        Ok(false) => Json(json!({
            "status": "not_found",
            "simulation_id": sim_id.to_string()
        })),
        Err(e) => Json(json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
}

//
// ───────────────────────────────────────────────
//   LIST ACTIVE SIMS
//   GET /api/simulations/runtime/list
// ───────────────────────────────────────────────
//
#[axum::debug_handler]
pub async fn list_sims(
    State(app): State<AppState>
) -> impl IntoResponse {

    let mgr = app.sim_manager.read().await;

    match mgr.list().await {
        Ok(sims) => Json(json!({
            "active_simulations": sims
                .into_iter()
                .map(|id| id.to_string()) // FIXED
                .collect::<Vec<_>>()
        })),
        Err(e) => Json(json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
}

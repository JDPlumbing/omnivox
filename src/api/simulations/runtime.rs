use axum::{
    extract::{State, Path},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use axum::http::StatusCode;

use crate::shared::app_state::AppState;
use crate::sim::simulations::simulation_config::SimulationConfig;
use crate::core::id::{WorldId, UvoxRegionId, UserId};
use crate::supabasic::worlds::WorldRow;
//
// ───────────────────────────────────────────────
//   START SIM
//   POST /api/simulations/runtime/start
// ───────────────────────────────────────────────
//
pub async fn start_sim(
    State(state): State<AppState>,
) -> impl IntoResponse 
{
    let cfg = SimulationConfig::basic(
        WorldId(0),
        UvoxRegionId::default(),
        UserId::zero(),
    );

    let mut mgr = state.sim_manager.write().await;

    // Fetch world from DB
    let world_record = match WorldRow::fetch(&state.supa, cfg.world_id).await {
        Ok(w) => w,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() }))
            );
        }
    };

    match mgr.start(cfg, world_record).await {
        Ok(api_id) => (
            StatusCode::OK,
            Json(json!({
                "simulation_id": api_id,
                "status": "ok"
            })),
        ),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

//
// ───────────────────────────────────────────────
//   TICK SIM
//   POST /api/simulations/runtime/{id}/tick
// ───────────────────────────────────────────────
//
pub async fn tick_sim(
    State(state): State<AppState>,
    Path(id): Path<String>,     // <-- no parsing, just raw hashed ID
) -> impl IntoResponse {

    let mut mgr = state.sim_manager.write().await;

    match mgr.tick(id.clone()).await {
        Ok(events) => Json(json!({
            "status": "ok",
            "simulation_id": id,
            "event_count": events.len(),
            "events": events,
        })),
        Err(e) => Json(json!({
            "status": "error",
            "message": e.to_string(),
        })),
    }
}

//
// ───────────────────────────────────────────────
//   STOP SIM
//   POST /api/simulations/runtime/{id}/stop
// ───────────────────────────────────────────────
//
pub async fn stop_sim(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {

    let mut mgr = state.sim_manager.write().await;

    match mgr.stop(id.clone()).await {
        Ok(true) => Json(json!({
            "status": "stopped",
            "simulation_id": id,
        })),
        Ok(false) => Json(json!({
            "status": "not_found",
            "simulation_id": id,
        })),
        Err(e) => Json(json!({
            "status": "error",
            "message": e.to_string(),
        })),
    }
}

//
// ───────────────────────────────────────────────
//   LIST SIMS
//   GET /api/simulations/runtime/list
// ───────────────────────────────────────────────
//
pub async fn list_sims(
    State(state): State<AppState>,
) -> impl IntoResponse {

    let mgr = state.sim_manager.read().await;

    match mgr.list().await {
        Ok(list) => Json(json!({
            "active_simulations": list,
        })),
        Err(e) => Json(json!({
            "status": "error",
            "message": e.to_string(),
        })),
    }
}

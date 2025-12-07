use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::shared::app_state::AppState;
use crate::core::id::SimulationId;

/// POST /api/simulations/run/:id
/// Run a single tick of the in-memory simulation
pub async fn run_simulation(
    State(app): State<AppState>,
    Path(sim_id): Path<String>,
) -> impl IntoResponse {

    // ---------------------------
    // Parse SimulationId
    // ---------------------------
    let sim_id: SimulationId = match sim_id.parse() {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": format!("Invalid simulation ID: {e}") })),
            );
        }
    };

    // ---------------------------
    // Acquire SimulationManager
    // ---------------------------
    let mut mgr = app.sim_manager.write().await;

    // ---------------------------
    // Execute ECS tick
    // ---------------------------
    let events = match mgr.tick(sim_id).await {
        Ok(ev) => ev,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Tick failed: {e:?}") })),
            );
        }
    };

    // ---------------------------
    // Build response
    // ---------------------------
    (
        StatusCode::OK,
        Json(json!({
            "simulation_id": sim_id.to_string(),
            "event_count": events.len(),
            "events": events.into_iter().map(|e| json!({
                "entity_id": e.entity_id,
                "kind": format!("{:?}", e.kind),
                "payload": e.payload,
                "ticks": e.t.as_ns(),
                "world_id": e.world_id,
            })).collect::<Vec<_>>()
        }))
    )
}

use axum::{extract::{State, Path}, response::IntoResponse, Json};
use serde_json::{json, Value};
use crate::shared::app_state::AppState;
use crate::sim::World;
use crate::sim::systems::System;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn tick_sim(State(app): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let mut mgr = app.sim_manager.write().await;
    let events = mgr.tick(id).await.unwrap_or_default();
    Json(json!({
    "status": "ticked",
    "event_count": events.len(),
    "events": events
}))

}

/*
#[axum::debug_handler]
pub async fn start_sim(
    State(app): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let mut mgr = app.sim_manager.write().await;
    let id = payload
        .get("simulation_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok());

    match id {
        Some(sim_id) => match mgr.load_from_supabase(sim_id).await {
            Ok(_) => Json(json!({ "status": "loaded", "simulation_id": sim_id })),
            Err(e) => Json(json!({ "status": "error", "message": e.to_string() })),
        },
        None => Json(json!({ "status": "error", "message": "missing simulation_id" })),
    }
}
*/
#[axum::debug_handler]
pub async fn start_sim(State(app): State<AppState>) -> impl IntoResponse {
    tracing::info!("🔥 start_sim endpoint hit");
    let mut mgr = app.sim_manager.write().await;

    match mgr.start().await {
        Ok(sim_id) => Json(json!({
            "status": "started",
            "simulation_id": sim_id
        })),
        Err(e) => Json(json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
}

#[axum::debug_handler]
pub async fn stop_sim(State(app): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let mut mgr = app.sim_manager.write().await;
    match mgr.stop(id).await {
        Ok(true) => Json(json!({ "status": "stopped" })),
        Ok(false) => Json(json!({ "status": "not_found" })),
        Err(e) => Json(json!({ "status": "error", "message": e.to_string() })),
    }
}

#[axum::debug_handler]
pub async fn list_sims(State(app): State<AppState>) -> impl IntoResponse {
    let mgr = app.sim_manager.read().await;
    match mgr.list().await {
        Ok(sims) => Json(json!({ "active_simulations": sims })),
        Err(e) => Json(json!({ "status": "error", "message": e.to_string() })),
    }
}

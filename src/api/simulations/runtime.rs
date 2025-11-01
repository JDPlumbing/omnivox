use axum::{extract::{State, Path}, response::IntoResponse, Json};
use serde_json::{json, Value};
use crate::shared::app_state::AppState;
use crate::sim::world::World;
use crate::sim::systems::System;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn tick_sim(State(app): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let mut mgr = app.sim_manager.write().await;
    let events = mgr.tick(id).await.unwrap_or_default();
    Json(json!({ "status": "ticked", "event_count": events.len() }))
}

#[axum::debug_handler]
pub async fn start_sim(State(app): State<AppState>) -> impl IntoResponse {
    let mut mgr = app.sim_manager.write().await;
    match mgr.start().await {
        Ok(id) => Json(json!({ "status": "started", "simulation_id": id })),
        Err(e) => Json(json!({ "status": "error", "message": e.to_string() })),
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

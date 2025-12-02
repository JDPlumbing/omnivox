use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;
use chrono::Utc;

use crate::shared::app_state::AppState;
use crate::sim::entities::SimEntity;
use crate::supabasic::entity::EntityRecord;

use crate::core::tdt::sim_duration::SimDuration;


#[derive(Debug, Deserialize)]
pub struct SimulationInitRequest {
    pub world_id: i64,
    pub dt_seconds: Option<i64>,
    pub speed: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct SimulationInitResponse {
    pub status: String,
    pub simulation_id: Uuid,
    pub world_id: i64,
    pub entity_count: usize,
    pub event_count: usize,
}


pub async fn init_simulation(
    State(app): State<AppState>,
    Json(req): Json<SimulationInitRequest>,
) -> impl IntoResponse {

    let sim_id = Uuid::new_v4();

    // ---------------------------
    // Convert dt from seconds → SimDuration
    // ---------------------------
    let dt_secs = req.dt_seconds.unwrap_or(60);
    let dt = SimDuration::from_seconds(dt_secs.into());

    let speed = req.speed.unwrap_or(100.0);

    // ---------------------------
    // Store simulation config
    // ---------------------------
    let config_payload = json!({
        "simulation_id": sim_id,
        "world_id": req.world_id,
        "dt_ns": dt.as_ns(),         // ✔ correct field
        "speed": speed,
        "metadata": {}
    });

    if let Err(e) = app.supa
        .from("simulations")
        .insert(config_payload)
        .select("simulation_id")
        .execute()
        .await
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Simulation insert failed: {e:?}") }))
        );
    }

    // ---------------------------
    // Load SimEntities for this world
    // ---------------------------
    let rows: Vec<EntityRecord> = match app.supa
        .from("sim_entities")
        .select("*")
        .eq("world_id", &req.world_id.to_string())
        .execute_typed()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to load entities: {e:?}") }))
            );
        }
    };

    let entities: Vec<SimEntity> =
        rows.into_iter().filter_map(|r| SimEntity::try_from(r).ok()).collect();


    // ---------------------------
    // Produce Installed events
    // ---------------------------
    let install_t = Utc::now();
    let ticks = install_t.timestamp_nanos_opt().unwrap_or(0);

    let events: Vec<Value> = entities.iter().map(|ent| {
        json!({
            "simulation_id": sim_id,
            "entity_id": ent.id,
            "world_id": ent.world_id,
            "ticks": ticks,
            "kind": "Installed",
            "payload": { "source": "init_simulation" },
            "created_at": install_t
        })
    }).collect();


    let inserted = app.supa
        .from("events")
        .insert_raw(json!(events))
        .select("id")
        .execute()
        .await;

    let event_count = inserted
        .ok()
        .and_then(|v| v.as_array().map(|a| a.len()))
        .unwrap_or(0);


    // ---------------------------
    // SUCCESS RESPONSE
    // Must return (StatusCode, Json<Value>)
    // ---------------------------
    (
        StatusCode::OK,
        Json(json!({
            "status": "initialized",
            "simulation_id": sim_id,
            "world_id": req.world_id,
            "entity_count": entities.len(),
            "event_count": event_count
        }))
    )
}

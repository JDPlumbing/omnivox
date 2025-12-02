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
use crate::sim::simulations::simulation_config::SimulationConfig;

use crate::core::tdt::sim_time::SimTime;


#[derive(Debug, Deserialize)]
pub struct RunSimulationRequest {
    pub simulation_id: Uuid,
}


pub async fn run_simulation(
    State(app): State<AppState>,
    Json(req): Json<RunSimulationRequest>,
) -> impl IntoResponse {

    // ---------------------------
    // Load simulation config
    // ---------------------------
    let config: SimulationConfig = match app.supa
        .from("simulations")
        .select("*")
        .eq("simulation_id", &req.simulation_id.to_string())
        .single_typed()
        .await
    {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": format!("Simulation not found: {e:?}") }))
            )
        }
    };


    // ---------------------------
    // Fetch entities in world
    // ---------------------------
    let rows: Vec<EntityRecord> = match app.supa
        .from("sim_entities")
        .select("*")
        .eq("world_id", &config.world_id.to_string())
        .execute_typed()
        .await
    {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to load entities: {e:?}") }))
            )
        }
    };

    let entities: Vec<SimEntity> =
        rows.into_iter().filter_map(|r| SimEntity::try_from(r).ok()).collect();


    // ---------------------------
    // Compute next timestep
    // ---------------------------
    let now = Utc::now();
    let ticks = now.timestamp_nanos_opt().unwrap_or(0);
    let next_t = SimTime::from_ns(ticks.into()); //figure out why ticks.into() was i64 at all


    // ---------------------------
    // Generate "Tick" events
    // ---------------------------
    let dt_seconds = config.dt.seconds_f64();

    

    let events: Vec<Value> = entities.iter().map(|ent| {
        json!({
            "simulation_id": config.simulation_id,
            "entity_id": ent.id,
            "world_id": ent.world_id,
            "ticks": next_t.as_ns(),
            "kind": "Tick",
            "payload": {
                "dt_seconds": dt_seconds,
                "source": "run_simulation"
            },
            "created_at": now
        })
    }).collect();


    // Insert into DB
    let inserted = match app.supa
        .from("events")
        .insert_raw(json!(events))
        .select("id")
        .execute()
        .await
    {
        Ok(v) => v.as_array().map(|a| a.len()).unwrap_or(0),
        Err(_) => 0,
    };


    // ---------------------------
    // SUCCESS RESPONSE
    // MUST return (StatusCode, Json<Value>)
    // ---------------------------
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "new_events": inserted,
            "next_time": next_t.as_ns()
        }))
    )
}

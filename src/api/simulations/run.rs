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
use crate::supabasic::objex::ObjexRecord;

#[derive(Debug, Deserialize)]
pub struct RunSimulationRequest {
    pub simulation_id: Uuid,
    pub frame_id: i64,
}

#[derive(Debug, Serialize)]
pub struct RunSimulationResponse {
    pub status: String,
    pub total_objects: usize,
    pub new_events: usize,
    pub sample: Vec<Value>,
}

pub async fn run_simulation(
    State(app): State<AppState>,
    Json(req): Json<RunSimulationRequest>,
) -> impl IntoResponse {
    // 1️⃣ Fetch all objects for this frame
    let objs: Vec<ObjexRecord> = match app
        .supa
        .from("objex_entities")
        .select("entity_id, name, frame_id")
        .eq("frame_id", &req.frame_id.to_string())
        .execute_typed()
        .await
    {
        Ok(list) => list,
        Err(e) => {
            eprintln!("❌ Failed to load objects for frame {}: {:?}", req.frame_id, e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to load objects: {e:?}") })),
            )
                .into_response();
        }
    };

    if objs.is_empty() {
        eprintln!("⚠️ No objects found for frame {}", req.frame_id);
    }

    // 2️⃣ Generate new events
    let now = Utc::now();
    let mut events: Vec<Value> = Vec::new();

    for o in &objs {
        // Randomly generate degradation/failure
        if rand::random::<f32>() < 0.1 {
            events.push(json!({
                "id": Value::Null,
                "simulation_id": req.simulation_id,
                "entity_id": o.entity_id,
                "frame_id": req.frame_id,
                "r_um": 0,
                "lat_code": 0,
                "lon_code": 0,
                "ticks": 0,
                "timestamp": now,
                "kind": "DegradationStart",
                "payload": {
                    "source": "simulation_run",
                    "severity": "minor"
                },
                "created_at": Value::Null
            }));
        }

        if rand::random::<f32>() < 0.01 {
            events.push(json!({
                "id": Value::Null,
                "simulation_id": req.simulation_id,
                "entity_id": o.entity_id,
                "frame_id": req.frame_id,
                "r_um": 0,
                "lat_code": 0,
                "lon_code": 0,
                "ticks": 0,
                "timestamp": now,
                "kind": "Failure",
                "payload": {
                    "source": "simulation_run",
                    "severity": "major"
                },
                "created_at": Value::Null
            }));
        }
    }

    println!(
        "🧮 Generated {} events for simulation {}",
        events.len(),
        req.simulation_id
    );

    // 3️⃣ Insert events into Supabase
    let inserted = match app
        .supa
        .from("events")
        .insert(events.clone())
        .select("id")
        .execute()
        .await
    {
        Ok(v) => v.as_array().map(|a| a.len()).unwrap_or(0),
        Err(e) => {
            eprintln!("⚠️ Failed to insert events: {:?}", e);
            0
        }
    };

    // ✅ Done
    Json(RunSimulationResponse {
        status: "ok".to_string(),
        total_objects: objs.len(),
        new_events: inserted,
        sample: events.iter().take(5).cloned().collect(),
    })
    .into_response()
}

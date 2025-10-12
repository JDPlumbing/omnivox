use axum::{
    extract::{Path, State},
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use serde_json::json;
use chrono::Utc;

use crate::shared::app_state::AppState;
use crate::supabasic::simulations::{UpdateSimulation, SimulationRow};
use crate::supabasic::events::EventRow;
use crate::supabasic::objex::ObjectRecord;


#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationDto {
    pub simulation_id: Uuid,
    pub user_owner_id: Option<Uuid>,
    pub anon_owner_id: Option<Uuid>,
    pub tick_rate: i64,
    pub frame_id: i64,
    pub last_saved: Option<String>,
    #[serde(default)]
    pub events: Vec<EventRow>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewSimulation {
    pub simulation_id: Option<Uuid>, // optional; generated if missing
    pub frame_id: i64,
    pub tick_rate: i64,
    pub last_saved: Option<chrono::DateTime<chrono::Utc>>,
    pub metadata: Option<serde_json::Value>,
    pub user_owner_id: Option<Uuid>,
    pub anon_owner_id: Option<Uuid>,
}


impl From<SimulationRow> for SimulationDto {
    fn from(row: SimulationRow) -> Self {
        SimulationDto {
            simulation_id: row.simulation_id,
            user_owner_id: row.user_owner_id,
            anon_owner_id: row.anon_owner_id,
            tick_rate: row.tick_rate,
            frame_id: row.frame_id,
            last_saved: row.last_saved.map(|dt| dt.to_rfc3339()),
            events: vec![],
        }
    }
}
/// POST /api/simulations
pub async fn create_simulation(
    State(app): State<AppState>,
    Json(payload): Json<NewSimulation>,
) -> impl IntoResponse {
    let new_id = Uuid::new_v4();

    // ✅ Build exactly the JSON that works via curl
    let insert_payload = json!([{
        "simulation_id": new_id,
        "frame_id": payload.frame_id,
        "tick_rate": payload.tick_rate,
        "last_saved": payload.last_saved,
        "metadata": json!({}),
        "user_owner_id": payload.user_owner_id,
        "anon_owner_id": payload.anon_owner_id
    }]);

    println!(
        "🧩 FINAL JSON TO SUPABASE:\n{}",
        serde_json::to_string_pretty(&insert_payload).unwrap()
    );

    // ✅ Use shared AppState’s Supabase client
    let result = app
        .supa
        .from("simulations")
        .insert_raw(insert_payload)
        .select("*")
        .execute_typed::<SimulationRow>()
        .await;

    match result {
        Ok(rows) => Json(json!({ "status": "ok", "inserted": rows })).into_response(),
        Err(e) => {
            eprintln!("❌ Error creating simulation: {:?}", e);
            (
                StatusCode::BAD_REQUEST,
                format!("Insert failed: {e:?}"),
            )
            .into_response()
        }
    }
}

// ========================================================
// GET /api/simulations/:id
// ========================================================
pub async fn get_simulation(
    State(app): State<AppState>,
    Path(sim_id): Path<Uuid>,
) -> impl IntoResponse {
    match SimulationRow::get(&app.supa, sim_id).await {
        Ok(sim) => {
            let mut dto = SimulationDto::from(sim);

            match EventRow::list_for_sim(&app.supa, &dto.simulation_id).await {
                Ok(events) => dto.events = events,
                Err(e) => eprintln!(
                    "⚠️ Warning: could not load events for sim {}: {:?}",
                    dto.simulation_id, e
                ),
            }

            Json(dto).into_response()
        }
        Err(e) => {
            eprintln!("Error fetching simulation {}: {:?}", sim_id, e);
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}

// ========================================================
// GET /api/simulations
// ========================================================
pub async fn list_simulations(State(app): State<AppState>) -> impl IntoResponse {
    match SimulationRow::list(&app.supa).await {
        Ok(sims) => {
            let dto_list: Vec<SimulationDto> =
                sims.into_iter().map(SimulationDto::from).collect();
            Json(dto_list).into_response()
        }
        Err(e) => {
            eprintln!("Error listing simulations: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}

// ========================================================
// POST /api/simulations/:id/seed
// ========================================================

// TODO: this function does not work and is pre-refactor. maybe needs to be repurposed
pub async fn seed_simulation(
    State(app): State<AppState>,
    Path(sim_id): Path<Uuid>,
) -> impl IntoResponse {
    // 1️⃣ Get simulation's world frame
    let sim: serde_json::Value = match app
        .supa
        .from("simulations")
        .select("frame_id")
        .eq("simulation_id", &sim_id.to_string())
        .single()
        .await
    {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to get simulation: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "error getting simulation" })),
            )
                .into_response();
        }
    };
    let frame_id = sim["frame_id"].as_i64().unwrap_or(0);

    // 2️⃣ Get world objects
    let objs: Vec<ObjectRecord> = match app
        .supa
        .from("objex_entities")
        .select("entity_id, name, shape, material_name, material_kind, frame_id")
        .eq("frame_id", &frame_id.to_string())
        .execute_typed()
        .await
    {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Failed to fetch objects: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "error fetching objects" })),
            )
                .into_response();
        }
    };

    // 3️⃣ Create spawn events
    let events: Vec<serde_json::Value> = objs
        .into_iter()
        .map(|o| {
            json!({
                "simulation_id": sim_id,
                "entity_id": o.entity_id,
                "frame_id": frame_id,
                "ticks": 0,
                "timestamp": Utc::now(),
                "kind": "Spawn",
                "move_offset": null,
                "payload": null,
            })
        })
        .collect();

    match app.supa.from("events").insert(events).select("*").execute().await {
        Ok(res) => Json(json!({ "status": "ok", "spawned": res })).into_response(),
        Err(e) => {
            eprintln!("Insert failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "insert failed" })),
            )
                .into_response()
        }
    }
}

// ========================================================
// POST /api/simulations/init
// ========================================================
#[derive(Debug, Deserialize)]
pub struct SimulationInitRequest {
    pub frame_id: i64,
    pub uvoxid: String,
    pub radius_um: Option<i64>,
    pub tick_rate: Option<i64>,
    pub anon_owner_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct SimulationInitResponse {
    pub status: String,
    pub simulation_id: Uuid,
    pub frame_id: i64,
    pub object_count: usize,
    pub spawned_event_count: usize,
}

pub async fn init_simulation(
    State(app): State<AppState>,
    Json(req): Json<SimulationInitRequest>,
) -> impl IntoResponse {
    // 1️⃣ Create a new simulation record
    let sim_id = Uuid::new_v4();
    let insert_res = app
        .supa
        .from("simulations")
        .insert(json!([{
            "simulation_id": sim_id,
            "frame_id": req.frame_id,
            "tick_rate": req.tick_rate.unwrap_or(1),
            "anon_owner_id": req.anon_owner_id,
        }]))
        .select("simulation_id")
        .execute()
        .await;

    if let Err(e) = insert_res {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Simulation insert failed: {e:?}") })),
        )
            .into_response();
    }

    // 2️⃣ Get nearby objects (simplified for now)
    let objs: Vec<ObjectRecord> = match app
        .supa
        .from("objex_entities")
        .select("entity_id, frame_id")
        .eq("frame_id", &req.frame_id.to_string())
        .execute_typed()
        .await
    {
        Ok(list) => list,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Fetch objects failed: {e:?}") })),
            )
                .into_response()
        }
    };

    // 3️⃣ Create spawn events
    let events: Vec<_> = objs
        .iter()
        .map(|o| {
            json!({
                "simulation_id": sim_id,
                "entity_id": o.entity_id,
                "frame_id": req.frame_id,
                "ticks": 0,
                "timestamp": Utc::now(),
                "kind": "Spawn",
            })
        })
        .collect();

    let insert_events = app
        .supa
        .from("events")
        .insert(events.clone())
        .select("id")
        .execute()
        .await;

    let spawned_count = match insert_events {
        Ok(v) => v.as_array().map(|a| a.len()).unwrap_or(0),
        Err(_) => 0,
    };

    Json(SimulationInitResponse {
        status: "initialized".to_string(),
        simulation_id: sim_id,
        frame_id: req.frame_id,
        object_count: objs.len(),
        spawned_event_count: spawned_count,
    })
    .into_response()
}

// ========================================================
// PUT /api/simulations/{id}
// ========================================================
pub async fn update_simulation(
    State(app): State<AppState>,
    Path(sim_id): Path<Uuid>,
    Json(payload): Json<UpdateSimulation>,
) -> impl IntoResponse {
    let update_json = json!({
        "frame_id": payload.frame_id,
        "tick_rate": payload.tick_rate,
        "last_saved": payload.last_saved,
        "metadata": payload.metadata,
        "user_owner_id": payload.user_owner_id,
        "anon_owner_id": payload.anon_owner_id
    });

    let result = app
        .supa
        .from("simulations")
        .update(update_json)
        .eq("simulation_id", &sim_id.to_string())
        .select("*")
        .execute_typed::<SimulationRow>()
        .await;


    match result {
        Ok(rows) => Json(rows).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Update failed: {e:?}") })),
        )
            .into_response(),
    }
}

// ========================================================
// PATCH /api/simulations/{id}
// ========================================================
pub async fn patch_simulation(
    State(app): State<AppState>,
    Path(sim_id): Path<Uuid>,
    Json(changes): Json<serde_json::Value>,
) -> impl IntoResponse {
    let result = app
        .supa
        .from("simulations")
        .eq("simulation_id", &sim_id.to_string())
        .update(changes)
        .select("*")
        .execute_typed::<SimulationRow>()
        .await;

    match result {
        Ok(rows) => Json(json!({ "patched": rows })).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Patch failed: {e:?}") })),
        )
            .into_response(),
    }
}

// ========================================================
// DELETE /api/simulations/{id}
// ========================================================
pub async fn delete_simulation(
    State(app): State<AppState>,
    Path(sim_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = app
        .supa
        .from("simulations")
        .eq("simulation_id", &sim_id.to_string())
        .delete()
        .execute()
        .await;

    match result {
        Ok(_) => Json(json!({ "status": "deleted", "id": sim_id })).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Delete failed: {e:?}") })),
        )
            .into_response(),
    }
}

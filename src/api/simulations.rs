use axum::{
    extract::{Path, State},
    response::IntoResponse,
    http::StatusCode,
    Json,
};

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use serde_json::{ json, Value };
use crate::tdt::core::TimeDelta;
use chrono::{Utc, TimeZone};

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
// POST /api/simulations/init
// ========================================================

#[derive(Debug, Deserialize)]
pub struct SimulationInitRequest {
    pub property_id: Uuid,
    pub frame_id: Option<i64>,
    pub tick_rate: Option<i64>,
    pub anon_owner_id: Option<Uuid>,
    pub user_owner_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct SimulationInitResponse {
    pub status: String,
    pub simulation_id: Uuid,
    pub frame_id: i64,
    pub object_count: usize,
    pub installed_event_count: usize,
}

pub async fn init_simulation(
    State(app): State<AppState>,
    Json(req): Json<SimulationInitRequest>,
) -> impl IntoResponse {
    // 1️⃣ Create the new simulation record
    let sim_id = Uuid::new_v4();
    let frame_id = req.frame_id.unwrap_or(0);

    let (user_owner_id, anon_owner_id) = match (req.user_owner_id, req.anon_owner_id) {
        (Some(uid), _) => (json!(uid), serde_json::Value::Null),
        (None, Some(aid)) => (serde_json::Value::Null, json!(aid)),
        (None, None) => (serde_json::Value::Null, json!(Uuid::new_v4())),
    };

    let insert_payload = json!({
        "simulation_id": sim_id,
        "frame_id": frame_id,
        "tick_rate": req.tick_rate.unwrap_or(1),
        "last_saved": serde_json::Value::Null,
        "metadata": json!({}),
        "user_owner_id": user_owner_id,
        "anon_owner_id": anon_owner_id,
    });

    println!("🧩 FINAL JSON TO SUPABASE:\n{}", serde_json::to_string_pretty(&insert_payload).unwrap());

    let insert_res = app
        .supa
        .from("simulations")
        .insert(insert_payload)
        .select("simulation_id")
        .execute()
        .await;

    if let Err(e) = insert_res {
        eprintln!("❌ Simulation insert failed: {:?}", e);
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Simulation insert failed: {e:?}") })),
        )
            .into_response();
    }

    // 2️⃣ Fetch property info → to establish tick 0
    let property: serde_json::Value = match app
        .supa
        .from("properties")
        .select("year_built")
        .eq("property_id", &req.property_id.to_string())
        .single()
        .await
    {
        Ok(v) => v,
        Err(e) => {
            eprintln!("❌ Failed to fetch property info: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to fetch property info: {e:?}") })),
            )
                .into_response();
        }
    };

    let year_built = property["year_built"].as_i64().unwrap_or(2000);
    let install_time = Utc
        .with_ymd_and_hms(year_built as i32, 1, 1, 0, 0, 0)
        .unwrap();

    let lived_delta = TimeDelta::until_now(install_time);
    let lived_days = lived_delta.ticks("days");
    let lived_pretty = lived_delta.pretty(2);

    println!(
        "🏗️ Property built in {year_built} → lived for {} days ({})",
        lived_days, lived_pretty
    );

    // 3️⃣ Fetch all existing objects for this property
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
            eprintln!("❌ Failed to fetch objects: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Fetch objects failed: {e:?}") })),
            )
                .into_response();
        }
    };

    // 4️⃣ Create Installed events (tick 0 = construction)
    let events: Vec<serde_json::Value> = objs
        .iter()
        .map(|o| {
            json!({
                "simulation_id": sim_id,
                "entity_id": o.entity_id,
                "frame_id": frame_id,
                "ticks": 0, // tick 0 = year built
                "timestamp": install_time,
                "kind": "Installed",
                "move_offset": serde_json::Value::Null,
                "payload": json!({
                    "installed_at_year": year_built,
                    "elapsed_since_install_days": lived_days,
                    "elapsed_pretty": lived_pretty,
                    "source": "property_init"
                })
            })
        })
        .collect();

    println!("🧾 Final Installed events payload:\n{}", serde_json::to_string_pretty(&events).unwrap());

    // 5️⃣ Insert events
    let insert_events = app
        .supa
        .from("events")
        .insert(events.clone())
        .select("id")
        .execute()
        .await;

    let inserted_count = match insert_events {
        Ok(v) => v.as_array().map(|a| a.len()).unwrap_or(0),
        Err(e) => {
            eprintln!("⚠️ Failed to insert Installed events: {:?}", e);
            0
        }
    };

    // ✅ Done
    Json(SimulationInitResponse {
        status: "initialized".to_string(),
        simulation_id: sim_id,
        frame_id,
        object_count: objs.len(),
        installed_event_count: inserted_count,
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

// ========================================================
// POST /api/simulations/run
// ========================================================
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
    pub sample: Vec<serde_json::Value>,
}

pub async fn run_simulation(
    State(app): State<AppState>,
    Json(req): Json<RunSimulationRequest>,
) -> impl IntoResponse {
    let objs: Vec<ObjectRecord> = match app
        .supa
        .from("objex_entities")
        .select("entity_id, name, frame_id")
        .eq("frame_id", &req.frame_id.to_string())
        .execute_typed()
        .await
    {
        Ok(list) => list,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to load objects: {e:?}") })),
            )
                .into_response();
        }
    };

    let now = Utc::now();
    let mut events = Vec::new();

    for o in &objs {
        if rand::random::<f32>() < 0.1 {
            events.push(json!({
                "simulation_id": req.simulation_id,
                "entity_id": o.entity_id,
                "frame_id": req.frame_id,
                "timestamp": now,
                "kind": "DegradationStart"
            }));
        }
        if rand::random::<f32>() < 0.01 {
            events.push(json!({
                "simulation_id": req.simulation_id,
                "entity_id": o.entity_id,
                "frame_id": req.frame_id,
                "timestamp": now,
                "kind": "Failure"
            }));
        }
    }

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

    Json(RunSimulationResponse {
        status: "ok".to_string(),
        total_objects: objs.len(),
        new_events: inserted,
        sample: events.iter().take(5).cloned().collect(),
    })
    .into_response()
}

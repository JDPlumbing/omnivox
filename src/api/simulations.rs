use axum::{extract::Path, response::IntoResponse, Json};
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use serde_json::json;

use crate::supabasic::Supabase;
use crate::supabasic::simulations::SimulationRow;
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

/// GET /api/simulations/:id
pub async fn get_simulation(Path(sim_id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();
    match SimulationRow::get(&supa, sim_id).await {
        Ok(sim) => {
            let mut dto = SimulationDto::from(sim);
            match EventRow::list_for_sim(&supa, &dto.simulation_id).await {
                Ok(events) => dto.events = events,
                Err(e) => eprintln!("Warning: could not load events for sim {}: {:?}", dto.simulation_id, e),
            }
            Json(dto).into_response()
        }
        Err(e) => {
            eprintln!("Error fetching simulation {}: {:?}", sim_id, e);
            (StatusCode::NOT_FOUND, "not found").into_response()
        }
    }
}

/// GET /api/simulations
pub async fn list_simulations() -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();
    match SimulationRow::list(&supa).await {
        Ok(sims) => {
            let dto_list: Vec<SimulationDto> = sims.into_iter().map(SimulationDto::from).collect();
            Json(dto_list).into_response()
        }
        Err(e) => {
            eprintln!("Error listing simulations: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response()
        }
    }
}

/// POST /api/simulations/:id/seed
pub async fn seed_simulation(Path(sim_id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    // 1️⃣ Get simulation's world frame
    let sim: serde_json::Value = match supa
        .from("simulations")
        .select("frame_id")
        .eq("simulation_id", &sim_id.to_string())
        .single()
        .await
    {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to get simulation: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "error getting simulation").into_response();
        }
    };
    let frame_id = sim["frame_id"].as_i64().unwrap_or(0);

    // 2️⃣ Get world objects
    let objs: Vec<ObjectRecord> = match supa
        .from("objex_entities")
        .select("entity_id, name, shape, material_name, material_kind, frame_id")
        .eq("frame_id", &frame_id.to_string())
        .execute_typed()
        .await

    {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Failed to fetch objects: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "error fetching objects").into_response();
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
                "timestamp": chrono::Utc::now(),
                "kind": "Spawn",
                "move_offset": null,
                "payload": null,
            })
        })
        .collect();

    match supa.from("events").insert(events).select("*").execute().await {
        Ok(res) => Json(json!({ "status": "ok", "spawned": res })).into_response(),
        Err(e) => {
            eprintln!("Insert failed: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "insert failed").into_response()
        }
    }
}

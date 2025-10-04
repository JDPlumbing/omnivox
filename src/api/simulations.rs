// src/api/simulations.rs
use axum::{extract::Path, response::IntoResponse, Json};
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::supabasic::client::Supabase;
use crate::supabasic::simulations::SimulationRow;
use crate::supabasic::events::EventRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationDto {
    pub simulation_id: Uuid,
    pub user_owner_id: Option<Uuid>,
    pub anon_owner_id: Option<Uuid>,
    pub tick_rate: i64,
    pub frame_id: i64,
    pub last_saved: Option<String>, // safer string for frontend
    #[serde(default)]
    pub events: Vec<EventRow>, // only populated in get_simulation
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
            // hydrate with events
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
            let dto_list: Vec<SimulationDto> =
                sims.into_iter().map(SimulationDto::from).collect();
            Json(dto_list).into_response()
        }
        Err(e) => {
            eprintln!("Error listing simulations: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response()
        }
    }
}

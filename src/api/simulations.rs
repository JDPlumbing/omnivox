// src/api/simulations.rs
use axum::{extract::Path, response::IntoResponse, Json};
use axum::http::StatusCode;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::supabasic::client::Supabase;
use crate::supabasic::simulations::SimulationRow;

#[derive(Serialize, Deserialize)]
pub struct SimulationDto {
    pub simulation_id: Uuid,
    pub user_owner_id: Option<Uuid>,
    pub anon_owner_id: Option<Uuid>,
    pub tick_rate: i64,
    pub frame_id: i64,
    pub last_saved: Option<String>, // String for frontend safety
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
        }
    }
}

/// GET /api/simulations/:id
pub async fn get_simulation(Path(sim_id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();
    match SimulationRow::get(&supa, sim_id).await {
        Ok(sim) => Json(SimulationDto::from(sim)).into_response(),
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

// src/api/simulations.rs
use axum::{extract::Path, response::IntoResponse, Json, debug_handler};
use uuid::Uuid;
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};

use crate::supabasic::Supabase;
use crate::sim::simulation::Simulation;
use crate::sim::systems::System; // for type of systems
use crate::sim::error::OmnivoxError;

#[derive(Serialize, Deserialize)]
pub struct SimulationDto {
    pub simulation_id: Uuid,
    pub current_tick: i64,
    pub frame_id: i64,
    pub timeline_len: usize,
    pub systems: Vec<String>, // names only, e.g. ["movement", "decay"]
}

impl From<&Simulation> for SimulationDto {
    fn from(sim: &Simulation) -> Self {
        SimulationDto {
            simulation_id: sim.simulation_id,
            current_tick: sim.current_tick,
            frame_id: sim.world.frame_id,
            timeline_len: sim.timeline.len(),
            systems: sim.systems
                .iter()
                .map(|sys| sys.name().to_string()) // each System trait has a `name()` fn
                .collect(),
        }
    }
}

/// Get a simulation by ID

#[debug_handler]
pub async fn get_simulation(
    Path((sim_id,)): Path<(Uuid,)>,
) -> Result<Json<SimulationDto>, StatusCode> {
    let supa = Supabase::new_from_env()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let sim = Simulation::load_from_supabase(&supa, sim_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SimulationDto::from(&sim)))
}

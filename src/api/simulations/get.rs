use axum::{
    extract::{Path, State},
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;
use super::SimulationDto;
use crate::core::id::simulation_id::SimulationId;

use crate::shared::app_state::AppState;
use crate::supabasic::{simulations::SimulationRow, events::EventRow};

/// GET /api/simulations/:id
pub async fn get_simulation(
    State(app): State<AppState>,
    Path(sim_id): Path<SimulationId>,
) -> impl IntoResponse {
    match SimulationRow::get(&app.supa, sim_id).await {
        Ok(sim) => {
            let mut dto = SimulationDto::from(sim);

            match EventRow::list_for_sim(&app.supa, &dto.simulation_id).await {
                Ok(events) => dto.events = events,
                Err(e) => eprintln!(
                    "⚠️ Could not load events for simulation {}: {:?}",
                    dto.simulation_id, e
                ),
            }

            Json(dto).into_response()
        }

        Err(e) => {
            eprintln!("❌ Error fetching simulation {}: {:?}", sim_id, e);
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": format!("Simulation not found: {e:?}") })),
            )
                .into_response()
        }
    }
}

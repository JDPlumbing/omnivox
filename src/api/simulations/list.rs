use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde_json::json;
use super::SimulationDto;

use crate::shared::app_state::AppState;
use crate::supabasic::simulations::SimulationRow;

/// GET /api/simulations
pub async fn list_simulations(State(app): State<AppState>) -> impl IntoResponse {
    match SimulationRow::list(&app.supa).await {
        Ok(sims) => {
            let dto_list: Vec<SimulationDto> =
                sims.into_iter().map(SimulationDto::from).collect();

            println!("📜 Returning {} simulations", dto_list.len());

            Json(dto_list).into_response()
        }
        Err(e) => {
            eprintln!("❌ Error listing simulations: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to list simulations: {e:?}") })),
            )
                .into_response()
        }
    }
}

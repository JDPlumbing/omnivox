use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
//use serde::{Serialize, Deserialize};
use uuid::Uuid;
use serde_json::json;
use super::{SimulationDto, NewSimulation};

use crate::shared::app_state::AppState;
use crate::supabasic::simulations::SimulationRow;


/// POST /api/simulations
pub async fn create_simulation(
    State(app): State<AppState>,
    Json(payload): Json<NewSimulation>,
) -> impl IntoResponse {
    let new_id = payload.simulation_id.unwrap_or_else(Uuid::new_v4);

    let insert_payload = json!([{
        "simulation_id": new_id,
        "world_id": payload.world_id,
        "tick_rate": payload.tick_rate,
        "last_saved": payload.last_saved,
        "metadata": payload.metadata.clone().unwrap_or_else(|| json!({})),
        "user_owner_id": payload.user_owner_id,
        "anon_owner_id": payload.anon_owner_id
    }]);

    println!(
        "🧩 FINAL JSON TO SUPABASE:\n{}",
        serde_json::to_string_pretty(&insert_payload).unwrap()
    );

    let result = app
        .supa
        .from("simulations")
        .insert_raw(insert_payload)
        .select("*")
        .execute_typed::<SimulationRow>()
        .await;

    match result {
        Ok(rows) => {
            let inserted = rows.first().map(|r| r.simulation_id);
            Json(json!({
                "status": "ok",
                "simulation_id": inserted,
                "inserted": rows
            }))
            .into_response()
        }
        Err(e) => {
            eprintln!("❌ Error creating simulation: {:?}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": format!("Insert failed: {e:?}") })),
            )
                .into_response()
        }
    }
}

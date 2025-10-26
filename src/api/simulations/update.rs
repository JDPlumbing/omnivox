use axum::{
    extract::{Path, State},
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::shared::app_state::AppState;
use crate::supabasic::simulations::{SimulationRow, UpdateSimulation};

/// PUT /api/simulations/{id}
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
        Ok(rows) => {
            println!("🧩 Updated simulation {}", sim_id);
            Json(rows).into_response()
        }
        Err(e) => {
            eprintln!("❌ Update failed for simulation {}: {:?}", sim_id, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": format!("Update failed: {e:?}") })),
            )
                .into_response()
        }
    }
}

/// PATCH /api/simulations/{id}
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
        Ok(rows) => {
            println!("🩹 Patched simulation {}", sim_id);
            Json(json!({ "patched": rows })).into_response()
        }
        Err(e) => {
            eprintln!("❌ Patch failed for simulation {}: {:?}", sim_id, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": format!("Patch failed: {e:?}") })),
            )
                .into_response()
        }
    }
}

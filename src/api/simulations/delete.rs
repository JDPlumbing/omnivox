use axum::{
    extract::{Path, State},
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::shared::app_state::AppState;

/// DELETE /api/simulations/{id}
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
        Ok(_) => {
            println!("🗑️ Deleted simulation {}", sim_id);
            Json(json!({ "status": "deleted", "id": sim_id })).into_response()
        }
        Err(e) => {
            eprintln!("❌ Delete failed for simulation {}: {:?}", sim_id, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": format!("Delete failed: {e:?}") })),
            )
                .into_response()
        }
    }
}

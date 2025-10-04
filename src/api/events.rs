use axum::{extract::Path, response::IntoResponse, Json};
use axum::http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::events::EventRow;

/// GET /api/events/:simulation_id
/// Lists all events for a given simulation.
pub async fn list_events_for_sim(Path(sim_id): Path<Uuid>) -> impl IntoResponse {
    let supa = match Supabase::new_from_env() {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Supabase init failed: {e:?}")).into_response(),
    };

    match EventRow::list_for_sim(&supa, &sim_id).await {
        Ok(events) => Json(events).into_response(),
        Err(e) => {
            eprintln!("Error fetching events for simulation {}: {:?}", sim_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Fetch failed: {e:?}")).into_response()
        }
    }
}

/// POST /api/events
/// Insert a new event directly (generic).
pub async fn create_event(Json(payload): Json<EventRow>) -> impl IntoResponse {
    let supa = match Supabase::new_from_env() {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Supabase init failed: {e:?}")).into_response(),
    };

    match EventRow::create(&supa, &payload).await {
        Ok(inserted) => Json(json!({ "status": "ok", "inserted": inserted })).into_response(),
        Err(e) => {
            eprintln!("Error inserting event: {:?}", e);
            (StatusCode::BAD_REQUEST, format!("Insert failed: {e:?}")).into_response()
        }
    }
}

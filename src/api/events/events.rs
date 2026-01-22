use axum::{
    //debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::supabasic::events::EventRow;
use crate::shared::app_state::AppState;
use crate::core::id::simulation_id::SimulationId;
use crate::core::id::entity_id::EntityId;
use crate::core::id::world_id::WorldId;

// ------------------------------------------------------------
// LIST: all events
// ------------------------------------------------------------
pub async fn list_events(State(app): State<AppState>) -> impl IntoResponse {
    match app
        .supa
        .from("events")
        .select("*")
        .execute_typed::<EventRow>()
        .await
    {
        Ok(rows) => Json(rows).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Fetch failed: {e:?}") })),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// GET: single event by id
// ------------------------------------------------------------
pub async fn get_event(State(app): State<AppState>, Path(event_id): Path<Uuid>) -> impl IntoResponse {
    match EventRow::get(&app.supa, event_id).await {
        Ok(event) => Json(event).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": format!("Event {event_id} not found: {e:?}") })),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// POST: create event
// ------------------------------------------------------------
pub async fn create_event(State(app): State<AppState>, Json(payload): Json<EventRow>) -> impl IntoResponse {
    match EventRow::create(&app.supa, &payload).await {
        Ok(inserted) => Json(json!({ "status": "ok", "inserted": inserted })).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Insert failed: {e:?}") })),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// PUT: full update
// ------------------------------------------------------------
pub async fn update_event(
    State(app): State<AppState>,
    Path(event_id): Path<Uuid>,
    Json(updated): Json<EventRow>,
) -> impl IntoResponse {
    let result = app
        .supa
        .from("events")
        .eq("id", &event_id.to_string())
        .update(serde_json::to_value(updated).unwrap())
        .select("*")
        .execute_typed::<EventRow>()
        .await;

    match result {
        Ok(rows) => Json(json!({ "updated": rows })).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Update failed: {e:?}") })),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// PATCH: partial update
// ------------------------------------------------------------
pub async fn patch_event(
    State(app): State<AppState>,
    Path(event_id): Path<Uuid>,
    Json(changes): Json<Value>,
) -> impl IntoResponse {
    let result = app
        .supa
        .from("events")
        .eq("id", &event_id.to_string())
        .update(changes)
        .select("*")
        .execute_typed::<EventRow>()
        .await;

    match result {
        Ok(rows) => Json(json!({ "patched": rows })).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Patch failed: {e:?}") })),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// DELETE: remove event
// ------------------------------------------------------------
pub async fn delete_event(State(app): State<AppState>, Path(event_id): Path<Uuid>) -> impl IntoResponse {
    let result = app
        .supa
        .from("events")
        .eq("id", &event_id.to_string())
        .delete()
        .execute()
        .await;

    match result {
        Ok(_) => Json(json!({ "status": "deleted", "id": event_id })).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Delete failed: {e:?}") })),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// LIST: events for simulation
// ------------------------------------------------------------
pub async fn list_events_for_sim(State(app): State<AppState>, Path(sim_id): Path<SimulationId>) -> impl IntoResponse {
    match EventRow::list_for_sim(&app.supa, &sim_id).await {
        Ok(events) => Json(events).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Fetch failed: {e:?}") })),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// LIST: events for entity
// ------------------------------------------------------------
pub async fn list_events_for_entity(State(app): State<AppState>, Path(entity_id): Path<EntityId>) -> impl IntoResponse {
    match EventRow::list_for_entity(&app.supa, &entity_id).await {
        Ok(events) => Json(events).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Fetch failed: {e:?}") })),
        )
            .into_response(),
    }
}

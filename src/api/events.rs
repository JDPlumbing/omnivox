// src/api/events.rs
use axum::{extract::Path, response::IntoResponse, Json};
use axum::http::StatusCode;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::events::EventRow;

// ------------------------------------------------------------
// LIST: all events (optional global or filtered later)
// ------------------------------------------------------------
pub async fn list_events() -> impl IntoResponse {
    let supa = match Supabase::new_from_env() {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Supabase init failed: {e:?}")).into_response(),
    };

    match supa
        .from("events")
        .select("*")
        .execute_typed::<EventRow>()
        .await
    {
        Ok(rows) => Json(rows).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Fetch failed: {e:?}"),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// GET: single event by id
// ------------------------------------------------------------
pub async fn get_event(Path(event_id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    match EventRow::get(&supa, event_id).await {
        Ok(event) => Json(event).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            format!("Event {event_id} not found: {e:?}"),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// POST: create event (already works fine)
// ------------------------------------------------------------
pub async fn create_event(Json(payload): Json<EventRow>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    match EventRow::create(&supa, &payload).await {
        Ok(inserted) => Json(json!({ "status": "ok", "inserted": inserted })).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            format!("Insert failed: {e:?}"),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// PUT: full update event
// ------------------------------------------------------------
pub async fn update_event(Path(event_id): Path<Uuid>, Json(updated): Json<EventRow>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    let result = supa
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
            format!("Update failed: {e:?}"),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// PATCH: partial update
// ------------------------------------------------------------
pub async fn patch_event(Path(event_id): Path<Uuid>, Json(changes): Json<Value>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    let result = supa
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
            format!("Patch failed: {e:?}"),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// DELETE: remove event
// ------------------------------------------------------------
pub async fn delete_event(Path(event_id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    let result = supa
        .from("events")
        .eq("id", &event_id.to_string())
        .delete()
        .execute()
        .await;

    match result {
        Ok(_) => Json(json!({ "status": "deleted", "id": event_id })).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            format!("Delete failed: {e:?}"),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// LIST: events for simulation
// ------------------------------------------------------------
pub async fn list_events_for_sim(Path(sim_id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    match EventRow::list_for_sim(&supa, &sim_id).await {
        Ok(events) => Json(events).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Fetch failed: {e:?}"),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// LIST: events for entity
// ------------------------------------------------------------
pub async fn list_events_for_entity(Path(entity_id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    match EventRow::list_for_entity(&supa, &entity_id).await {
        Ok(events) => Json(events).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Fetch failed: {e:?}"),
        )
            .into_response(),
    }
}

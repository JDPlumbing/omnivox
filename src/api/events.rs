use axum::{
    extract::{Path, Json},
    response::IntoResponse,
};
use crate::supabasic::Supabase;
use crate::supabasic::events::EventRow;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventDto {
    pub id: Uuid,
    pub simulation_id: Uuid,
    pub entity_id: Uuid,
    pub frame_id: i64,
    pub r_um: i64,
    pub lat_code: i64,
    pub lon_code: i64,
    pub ticks: i64,
    pub kind: String,
}

impl From<EventRow> for EventDto {
    fn from(e: EventRow) -> Self {
        EventDto {
            id: e.id,
            simulation_id: e.simulation_id,
            entity_id: e.entity_id,
            frame_id: e.frame_id,
            r_um: e.r_um,
            lat_code: e.lat_code,
            lon_code: e.lon_code,
            ticks: e.ticks,
            kind: e.kind,
        }
    }
}

/// GET /simulations/:id/events
pub async fn list_events_handler(Path(sim_id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();
    match EventRow::list_for_sim(&supa, &sim_id).await {
        Ok(rows) => {
            let dto: Vec<EventDto> = rows.into_iter().map(EventDto::from).collect();
            Json(dto).into_response()
        }
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// GET /events/:id
pub async fn get_event_handler(Path(event_id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();
    match EventRow::get(&supa, event_id).await {
        Ok(event) => Json(EventDto::from(event)).into_response(),
        Err(e) => (axum::http::StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}

/// POST /events
pub async fn create_event_handler(Json(payload): Json<EventRow>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();
    match EventRow::create(&supa, &payload).await {
        Ok(event) => Json(EventDto::from(event)).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

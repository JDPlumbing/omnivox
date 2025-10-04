// src/api/worlds.rs
use axum::{extract::Path, response::IntoResponse, Json};
use axum::http::StatusCode;

use crate::supabasic::{Supabase};
use crate::supabasic::worlds::{WorldRow, NewWorld};
use crate::supabasic::events::EventRow;

/// DTO returned to clients
#[derive(serde::Serialize)]
pub struct WorldDto {
    pub frame_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub events: Vec<EventRow>,
}

impl From<WorldRow> for WorldDto {
    fn from(w: WorldRow) -> Self {
        Self {
            frame_id: w.frame_id,
            name: w.name,
            description: w.description,
            created_at: w.created_at,
            updated_at: w.updated_at,
            deleted_at: w.deleted_at,
            events: vec![], // filled later
        }
    }
}

/// GET /worlds
pub async fn list_worlds_handler() -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    match WorldRow::list(&supa).await {
        Ok(rows) => {
            let mut result = Vec::new();

            for row in rows {
                // fetch events for this frame_id
                let events = EventRow::list_for_frame(&supa, row.frame_id)
                .await
                .unwrap_or_default();


                let mut dto = WorldDto::from(row);
                dto.events = events;
                result.push(dto);
            }

            Json(result).into_response()
        }
        Err(e) => {
            eprintln!("Error listing worlds: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response()
        }
    }
}

/// GET /worlds/:frame_id
pub async fn get_world_handler(Path(frame_id): Path<i64>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    match WorldRow::get(&supa, frame_id).await {
        Ok(row) => {
            // also fetch events
            let events = EventRow::list_for_frame(&supa, row.frame_id)
                .await
                .unwrap_or_default();


            let mut dto = WorldDto::from(row);
            dto.events = events;

            Json(dto).into_response()
        }
        Err(e) => {
            eprintln!("Error fetching world {}: {:?}", frame_id, e);
            (StatusCode::NOT_FOUND, "world not found").into_response()
        }
    }
}

/// POST /worlds
pub async fn create_world_handler(Json(payload): Json<NewWorld>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    match WorldRow::create(&supa, &payload).await {
        Ok(row) => {
            let dto = WorldDto::from(row);
            (StatusCode::CREATED, Json(dto)).into_response()
        }
        Err(e) => {
            eprintln!("Error creating world: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error creating world").into_response()
        }
    }
}

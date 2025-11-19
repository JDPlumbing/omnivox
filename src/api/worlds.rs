use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::supabasic::worlds::{WorldRecord, NewWorld};
use crate::supabasic::events::EventRow;
use crate::shared::app_state::AppState;

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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct WorldUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl From<WorldRecord> for WorldDto {
    fn from(w: WorldRecord) -> Self {
        Self {
            frame_id: w.frame_id,
            name: w.name,
            description: w.description,
            created_at: w.created_at,
            updated_at: w.updated_at,
            deleted_at: w.deleted_at,
            events: vec![],
        }
    }
}

// ------------------------------------------------------------
// GET /worlds
// ------------------------------------------------------------
pub async fn list_worlds_handler(State(app): State<AppState>) -> impl IntoResponse {
    match WorldRecord::list(&app.supa).await {
        Ok(rows) => {
            let mut result = Vec::new();
            for row in rows {
                let events = EventRow::list_for_frame(&app.supa, row.frame_id)
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
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "error listing worlds", "details": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// GET /worlds/{frame_id}
// ------------------------------------------------------------
pub async fn get_world_handler(State(app): State<AppState>, Path(frame_id): Path<i64>) -> impl IntoResponse {
    match WorldRecord::get(&app.supa, frame_id).await {
        Ok(row) => {
            let events = EventRow::list_for_frame(&app.supa, row.frame_id)
                .await
                .unwrap_or_default();
            let mut dto = WorldDto::from(row);
            dto.events = events;
            Json(dto).into_response()
        }
        Err(e) => {
            eprintln!("Error fetching world {}: {:?}", frame_id, e);
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "world not found", "details": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// POST /worlds
// ------------------------------------------------------------
pub async fn create_world_handler(State(app): State<AppState>, Json(payload): Json<NewWorld>) -> impl IntoResponse {
    match WorldRecord::create(&app.supa, &payload).await {
        Ok(row) => {
            let dto = WorldDto::from(row);
            (StatusCode::CREATED, Json(dto)).into_response()
        }
        Err(e) => {
            eprintln!("Error creating world: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "error creating world", "details": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// PUT /worlds/{frame_id}
// ------------------------------------------------------------
pub async fn update_world_handler(
    State(app): State<AppState>,
    Path(frame_id): Path<i64>,
    Json(updated): Json<WorldUpdate>,
) -> impl IntoResponse {
    let payload = serde_json::to_value(&updated).unwrap();
    eprintln!("📦 PUT /worlds/{frame_id} payload: {}", payload);

    let result = app
        .supa
        .from("worlds")
        .eq("frame_id", &frame_id.to_string())
        .update(payload)
        .select("*")
        .execute_typed::<WorldRecord>()
        .await;

    match result {
        Ok(mut rows) => {
            if rows.is_empty() {
                return (StatusCode::NOT_FOUND, "No row updated").into_response();
            }
            Json(json!({ "updated": rows.remove(0) })).into_response()
        }
        Err(e) => {
            eprintln!("❌ Error updating world {}: {:?}", frame_id, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Update failed", "details": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// PATCH /worlds/{frame_id}
// ------------------------------------------------------------
pub async fn patch_world_handler(
    State(app): State<AppState>,
    Path(frame_id): Path<i64>,
    Json(changes): Json<serde_json::Value>,
) -> impl IntoResponse {
    let result = app
        .supa
        .from("worlds")
        .eq("frame_id", &frame_id.to_string())
        .update(changes)
        .select("*")
        .execute_typed::<WorldRecord>()
        .await;

    match result {
        Ok(rows) => Json(json!({ "patched": rows })).into_response(),
        Err(e) => {
            eprintln!("Error patching world {}: {:?}", frame_id, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Patch failed", "details": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// DELETE /worlds/{frame_id}
// ------------------------------------------------------------
pub async fn delete_world_handler(State(app): State<AppState>, Path(frame_id): Path<i64>) -> impl IntoResponse {
    let result = app
        .supa
        .from("worlds")
        .eq("frame_id", &frame_id.to_string())
        .delete()
        .execute()
        .await;

    match result {
        Ok(_) => Json(json!({ "status": "deleted", "frame_id": frame_id })).into_response(),
        Err(e) => {
            eprintln!("Error deleting world {}: {:?}", frame_id, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Delete failed", "details": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}

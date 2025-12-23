use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::supabasic::worlds::{WorldRow, NewWorldRow};
use crate::supabasic::events::EventRow;
use crate::shared::app_state::AppState;
use crate::core::id::WorldId;

/// DTO returned to clients
#[derive(serde::Serialize)]
pub struct WorldDto {
    pub world_id: WorldId,
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

impl From<WorldRow> for WorldDto {
    fn from(w: WorldRow) -> Self {
        Self {
            world_id: w.world_id,
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
    match WorldRow::list(&app.supa).await {
        Ok(rows) => {
            let mut result = Vec::new();
            for row in rows {
                let events = EventRow::list_for_world(&app.supa, row.world_id)
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
// GET /worlds/{world_id}
// ------------------------------------------------------------
pub async fn get_world_handler(State(app): State<AppState>, Path(world_id): Path<WorldId>) -> impl IntoResponse {
    match WorldRow::get(&app.supa, world_id).await {
        Ok(row) => {
            let events = EventRow::list_for_world(&app.supa, row.world_id)
                .await
                .unwrap_or_default();
            let mut dto = WorldDto::from(row);
            dto.events = events;
            Json(dto).into_response()
        }
        Err(e) => {
            eprintln!("Error fetching world {}: {:?}", world_id, e);
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
pub async fn create_world_handler(State(app): State<AppState>, Json(payload): Json<NewWorldRow>) -> impl IntoResponse {
    match WorldRow::create(&app.supa, &payload).await {
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
// PUT /worlds/{world_id}
// ------------------------------------------------------------
pub async fn update_world_handler(
    State(app): State<AppState>,
    Path(world_id): Path<WorldId>,
    Json(updated): Json<WorldUpdate>,
) -> impl IntoResponse {
    let payload = serde_json::to_value(&updated).unwrap();
    eprintln!("📦 PUT /worlds/{world_id} payload: {}", payload);

    let result = app
        .supa
        .from("worlds")
        .eq("world_id", &world_id.to_string())
        .update(payload)
        .select("*")
        .execute_typed::<WorldRow>()
        .await;

    match result {
        Ok(mut rows) => {
            if rows.is_empty() {
                return (StatusCode::NOT_FOUND, "No row updated").into_response();
            }
            Json(json!({ "updated": rows.remove(0) })).into_response()
        }
        Err(e) => {
            eprintln!("❌ Error updating world {}: {:?}", world_id, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Update failed", "details": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// PATCH /worlds/{world_id}
// ------------------------------------------------------------
pub async fn patch_world_handler(
    State(app): State<AppState>,
    Path(world_id): Path<WorldId>,
    Json(changes): Json<serde_json::Value>,
) -> impl IntoResponse {
    let result = app
        .supa
        .from("worlds")
        .eq("world_id", &world_id.to_string())
        .update(changes)
        .select("*")
        .execute_typed::<WorldRow>()
        .await;

    match result {
        Ok(rows) => Json(json!({ "patched": rows })).into_response(),
        Err(e) => {
            eprintln!("Error patching world {}: {:?}", world_id, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Patch failed", "details": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// DELETE /worlds/{world_id}
// ------------------------------------------------------------
pub async fn delete_world_handler(State(app): State<AppState>, Path(world_id): Path<WorldId>) -> impl IntoResponse {
    let result = app
        .supa
        .from("worlds")
        .eq("world_id", &world_id.to_string())
        .delete()
        .execute()
        .await;

    match result {
        Ok(_) => Json(json!({ "status": "deleted", "world_id": world_id })).into_response(),
        Err(e) => {
            eprintln!("Error deleting world {}: {:?}", world_id, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Delete failed", "details": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// GET /worlds/{world_id}/stats
// ------------------------------------------------------------
use serde::Serialize;
#[derive(Serialize)]
pub struct WorldStats {
    pub world_id: WorldId,
    pub entity_count: i64,
}

pub async fn get_world_stats(
    State(app): State<AppState>,
    Path(world_id): Path<WorldId>,
) -> impl IntoResponse {
    let args = json!({ "world_id": world_id.0 });

    // Call the RPC function
    match app.supa.rpc("count_entities", args).await {
        Ok(val) => {
            // RPC returns something like: [{ "count_entities": 123 }]
            let count = val
                .get(0)
                .and_then(|row| row.get("count_entities"))
                .and_then(|v| v.as_i64())
                .unwrap_or(0);

            Json(json!({
                "world_id": world_id.0,
                "entity_count": count,
            }))
            .into_response()
        }

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("{e:?}") })),
        )
            .into_response(),
    }
}

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::shared::app_state::AppState;
use crate::supabasic::entity::EntityRow;
use crate::core::id::world_id::WorldId;
use crate::supabasic::orm::DbModel;
use crate::core::EntityId;

// ------------------------------------------------------------
// POST /api/entities
// Create entities (DB-backed, EntityRow only)
// ------------------------------------------------------------
#[derive(Debug, serde::Deserialize)]
pub struct CreateEntity {
    pub world_id: WorldId,

    pub template: serde_json::Value,
    pub position: serde_json::Value,
    pub orientation: serde_json::Value,

    pub spawned_at: crate::core::SimTime,
    pub metadata: serde_json::Value,
}

pub async fn create_entities(
    State(app): State<AppState>,
    Json(payload): Json<Vec<CreateEntity>>,
) -> impl IntoResponse {
    let mut created = Vec::new();

    for req in payload {
        let row = EntityRow {
            row_id: None, // let DB generate UUID
            world_id: req.world_id,

           // objex_template_id: req.template,
            position: req.position,
            orientation: req.orientation,

            spawned_at: req.spawned_at,
            despawned_at: None,

            metadata: req.metadata,
        };

        match EntityRow::insert(&app.supa, &row).await {
            Ok(inserted) => created.push(inserted),
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": format!("{e:?}") })),
                )
                    .into_response();
            }
        }
    }

    Json(created).into_response()
}

// ------------------------------------------------------------
// GET /api/entities/:id
// ------------------------------------------------------------
pub async fn get_entity(
    State(app): State<AppState>,
    Path(entity_id): Path<Uuid>,
) -> impl IntoResponse {
    match EntityRow::fetch(&app.supa, EntityId(entity_id)).await {
        Ok(rec) => Json(rec).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": format!("{e:?}") })),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// GET /api/entities
// ------------------------------------------------------------
pub async fn list_entities(
    State(app): State<AppState>,
) -> impl IntoResponse {
    let rows = app
        .supa
        .from(EntityRow::table())
        .select("*")
        .execute();

    match rows.await {
        Ok(raw) => Json(raw).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("{e:?}") })),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// GET /api/entities/world/:world_id
// ------------------------------------------------------------
pub async fn list_entities_for_world(
    State(app): State<AppState>,
    Path(world_id): Path<WorldId>,
) -> impl IntoResponse {
    match EntityRow::list_for_world(&app.supa, world_id).await {
        Ok(rows) => Json(rows).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("{e:?}") })),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// DELETE /api/entities/:id
// ------------------------------------------------------------
pub async fn delete_entity(
    State(app): State<AppState>,
    Path(entity_id): Path<Uuid>,
) -> impl IntoResponse {
    match app
        .supa
        .from(EntityRow::table())
        .delete()
        .eq("row_id", &entity_id.to_string())
        .execute()
        .await
    {
        Ok(_) => Json(json!({ "deleted": entity_id })).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("{e:?}") })),
        )
            .into_response(),
    }
}

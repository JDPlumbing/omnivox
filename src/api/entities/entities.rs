use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::shared::app_state::AppState;
use crate::core::SimEntity;
use crate::supabasic::entity::EntityRow;
//use crate::core::objex::Objex;
use crate::core::id::world_id::WorldId;
use crate::core::CreateSimEntity;
use crate::core::UvoxId;
use crate::core::objex::Objex;
use crate::core::id::EntityId;

// ------------------------------------------------------------
// POST /api/entities
// Create a new SimEntity
// ------------------------------------------------------------
/*
pub async fn create_entity(
    State(app): State<AppState>,
    Json(entity): Json<SimEntity>,
) -> impl IntoResponse {
    let objex_id = Uuid::new_v4(); // optional: if you later add objex table

    match EntityRow::insert(&app.supa, &entity).await {
        Ok(rec) => Json(json!({ "row_id": rec.row_id })).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("{e:?}") })),
        )
            .into_response(),
    }
}
*/
pub async fn create_entities(
    State(app): State<AppState>,
    Json(payload): Json<Vec<CreateSimEntity>>,
) -> impl IntoResponse {
    let mut created = Vec::new();

    for req in payload {
        let entity = SimEntity {
            id: EntityId::provisional(0),
           // TODO: EntityId allocation must move to engine allocator
            // This provisional ID is replaced after DB insert

            world_id: req.world_id,
            template: req.template,
            position: req.position,
            orientation: req.orientation,
            spawned_at: req.spawned_at,
            despawned_at: None,
            metadata: req.metadata,
        };

        match EntityRow::insert(&app.supa, &entity).await {
            Ok(row) => created.push(row),
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
    match EntityRow::fetch(&app.supa, entity_id).await {
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
pub async fn list_entities(State(app): State<AppState>) -> impl IntoResponse {
    let rows = app
        .supa
        .from("sim_entities")
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
        .from("sim_entities")
        .delete()
        .eq("entity_id", &entity_id.to_string())
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

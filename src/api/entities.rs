use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::shared::app_state::AppState;
use crate::sim::entity::SimEntity;
use crate::supabasic::entity::EntityRecord;
//use crate::core::objex::Objex;
use crate::core::id::world_id::WorldId;

// ------------------------------------------------------------
// POST /api/entities
// Create a new SimEntity
// ------------------------------------------------------------
pub async fn create_entity(
    State(app): State<AppState>,
    Json(entity): Json<SimEntity>,
) -> impl IntoResponse {
    let blueprint_id = Uuid::new_v4(); // optional: if you later add blueprint table

    match EntityRecord::insert(&app.supa, &entity).await {
        Ok(rec) => Json(json!({ "id": rec.id })).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("{e:?}") })),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// GET /api/entities/:id
// ------------------------------------------------------------
pub async fn get_entity(
    State(app): State<AppState>,
    Path(entity_id): Path<Uuid>,
) -> impl IntoResponse {
    match EntityRecord::fetch(&app.supa, entity_id).await {
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
    match EntityRecord::list_for_world(&app.supa, world_id).await {
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

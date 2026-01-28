use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::shared::app_state::AppState;
use crate::engine::entity::entity_engine::EntityEngine;
use crate::core::EntityId;
use crate::api::entities::dtos::set_world::SetWorldDto;

pub async fn set_entity_world(
    State(app): State<AppState>,
    Path(entity_uuid): Path<Uuid>,
    Json(payload): Json<SetWorldDto>,
) -> Json<serde_json::Value> {
    let entity_id = EntityId(entity_uuid);

    let mut store = app.entity_store.write().await;
    let mut engine = EntityEngine::new(&mut store);

    engine.set_world(entity_id, payload.world_id);

    Json(serde_json::json!({
        "entity_id": entity_id.to_string(),
        "world_id": payload.world_id,
    }))
}

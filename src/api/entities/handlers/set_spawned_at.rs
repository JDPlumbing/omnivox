use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use serde_json::json;

use crate::shared::app_state::AppState;
use crate::engine::entity::entity_engine::EntityEngine;
use crate::core::EntityId;
use crate::core::components::spawned_at::SpawnedAt;
use crate::api::entities::dtos::set_spawned_at::SetSpawnedAtDto;

pub async fn set_entity_spawned_at(
    State(app): State<AppState>,
    Path(entity_uuid): Path<Uuid>,
    Json(payload): Json<SetSpawnedAtDto>,
) -> Json<serde_json::Value> {
    let entity_id = EntityId(entity_uuid);

    let mut store = app.entity_store.write().await;
    let mut engine = EntityEngine::new(&mut store);

    engine.set_spawned_at(entity_id, SpawnedAt { time: payload.time });

    Json(json!({
        "entity_id": entity_id.to_string(),
        "spawned_at": payload.time,
    }))
}

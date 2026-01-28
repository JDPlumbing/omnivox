use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use serde_json::json;

use crate::shared::app_state::AppState;
use crate::engine::entity::entity_engine::EntityEngine;
use crate::core::EntityId;
use crate::core::components::position::Position;
use crate::api::entities::dtos::set_position::SetPositionDto;

pub async fn set_entity_position(
    State(app): State<AppState>,
    Path(entity_uuid): Path<Uuid>,
    Json(payload): Json<SetPositionDto>,
) -> Json<serde_json::Value> {
    let entity_id = EntityId(entity_uuid);

    let mut store = app.entity_store.write().await;
    let mut engine = EntityEngine::new(&mut store);

    engine.set_position(entity_id, Position(payload.uvox));

    Json(json!({
        "entity_id": entity_id.to_string(),
        "position": payload.uvox,
    }))
}

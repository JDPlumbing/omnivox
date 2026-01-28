use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use serde_json::json;

use crate::shared::app_state::AppState;
use crate::engine::entity::entity_engine::{EntityEngine};
use crate::engine::entity::errors::DespawnError;
use crate::core::EntityId;
use crate::core::components::despawned_at::DespawnedAt;
use crate::api::entities::dtos::set_despawned_at::SetDespawnedAtDto;

pub async fn set_entity_despawned_at(
    State(app): State<AppState>,
    Path(entity_uuid): Path<Uuid>,
    Json(payload): Json<SetDespawnedAtDto>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let entity_id = EntityId(entity_uuid);

    let mut store = app.entity_store.write().await;
    let mut engine = EntityEngine::new(&mut store);

    match engine.set_despawned_at(entity_id, DespawnedAt { time: payload.time }) {
        Ok(_) => Ok(Json(json!({
            "entity_id": entity_id.to_string(),
            "despawned_at": payload.time,
        }))),

        Err(DespawnError::NotSpawned) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "entity has no spawned_at"
            })),
        )),

        Err(DespawnError::InvalidTime) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "despawned_at must be greater than spawned_at"
            })),
        )),
    }
}

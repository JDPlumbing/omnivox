use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use serde_json::json;

use crate::shared::app_state::AppState;
use crate::core::EntityId;
use crate::api::entities::dtos::get_entity::GetEntityResponse;

pub async fn get_entity(
    State(app): State<AppState>,
    Path(entity_uuid): Path<Uuid>,
) -> Result<Json<GetEntityResponse>, axum::http::StatusCode> {
    let entity_id = EntityId(entity_uuid);

    let store = app.entity_store.read().await;

    let mut components = serde_json::Map::new();

    // ---- Time component ----
    if let Some(time) = store.times.get(&entity_id) {
        components.insert(
            "time".to_string(),
            json!({
                "sim_time": time.sim_time.0
            }),
        );
    }
    // ---- Note component ----
    if let Some(note) = store.notes.get(&entity_id) {
        components.insert(
            "note".to_string(),
            json!({
                "text": note.text
            }),
        );
    }
    // ---- World membership ----
    if let Some(wm) = store.world_memberships.get(&entity_id) {
        components.insert(
            "world".to_string(),
            serde_json::json!({
                "world_id": wm.world_id,
            }),
        );
    }
    // ---- Position ----
    if let Some(position) = store.positions.get(&entity_id) {
        components.insert(
            "position".to_string(),
            serde_json::json!({
                "uvox": position.0,
            }),
        );
    }
    // ---- SpawnedAt ----
    if let Some(spawned) = store.spawned_ats.get(&entity_id) {
        components.insert(
            "spawned_at".to_string(),
            serde_json::json!({
                "time": spawned.time,
            }),
        );
    }
    // ---- DespawnedAt ----
    if let Some(despawned) = store.despawned_ats.get(&entity_id) {
        components.insert(
            "despawned_at".to_string(),
            serde_json::json!({
                "time": despawned.time,
            }),
        );
    }

    // If no components found → entity does not exist
    if components.is_empty() {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }

    Ok(Json(GetEntityResponse {
        entity_id: entity_id.to_string(),
        components: components.into(),
    }))
}

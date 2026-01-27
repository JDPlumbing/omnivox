use axum::{
    extract::{Path, State},
    Json,
};
use crate::shared::app_state::AppState;
use crate::core::WorldId;

pub async fn list_entities_in_world(
    State(app): State<AppState>,
    Path(world_id): Path<WorldId>,
) -> Json<Vec<String>> {
    let store = app.entity_store.read().await;

    let entities = store
        .world_memberships
        .iter()
        .filter(|(_, wm)| wm.world_id == world_id)
        .map(|(id, _)| id.to_string())
        .collect();

    Json(entities)
}

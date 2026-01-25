use axum::{
    extract::{State, Path},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use reqwest::StatusCode;

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::core::components::position::Position;
use crate::engine::world::load_world;
use crate::core::UvoxId;
use crate::core::UvoxQuat;
use crate::core::components::orientation::Orientation;
use crate::core::components::lifecycle::Lifecycle;
use crate::core::id::EntityId;

#[derive(Debug, Deserialize)]
pub struct SpawnWithPosition {
    pub position: UvoxId,
    pub orientation: Option<UvoxQuat>,
}

#[derive(Debug, Deserialize)]
pub struct SpawnEntityWithPosition {
    pub position: UvoxId,
}

pub async fn spawn_entity_with_position(
    State(app): State<AppState>,
    Path(world_id): Path<WorldId>,
    Json(req): Json<SpawnWithPosition>,
) -> Result<Json<EntityId>, (StatusCode, Json<serde_json::Value>)> {

    let mut worlds = app.worlds.write().await;

    let world = if let Some(w) = worlds.get_mut(&world_id) {
        w
    } else {
        let world = load_world(&app.supa, world_id).await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": format!("{e:?}") }))
                )
            })?;

        worlds.insert(world_id, world);
        worlds.get_mut(&world_id).unwrap()
    };

    let entity = world.spawn_entity();

    world.world_membership.insert(entity, world_id);
    world.positions.insert(entity, Position(req.position));

    if let Some(o) = req.orientation {
        world.orientations.insert(entity, Orientation(o));
    }

    world.lifecycles.insert(entity, Lifecycle {
        spawned_at: world.sim_time,
        despawned_at: None,
    });

    Ok(Json(entity))
}

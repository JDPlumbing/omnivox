use axum::{extract::{Path, State}, response::IntoResponse, Json};
use axum::http::StatusCode;


use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::api::worlds::dtos::world::WorldStatsDto;

pub async fn get_world_stats_handler(
    State(app): State<AppState>,
    Path(world_id): Path<WorldId>,
) -> impl IntoResponse {
    match app.world_source.world_stats(world_id).await {
        Ok(stats) => Json(WorldStatsDto {
            world_id: stats.world_id,
            entity_count: stats.entity_count,
        })
        .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
        .into_response(),
    }

}

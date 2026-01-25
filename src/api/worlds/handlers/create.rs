use crate::supabasic::worlds::NewWorldRow;
use crate::core::id::WorldId;
use crate::api::worlds::dtos::world::WorldDto;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use crate::shared::app_state::AppState;
use serde_json::to_value;
use serde_json::Value;
use crate::core::world::presets::earth_v0;
use serde::Deserialize;
use crate::api::worlds::payloads::CreateWorldPayload;

pub async fn create_world_handler(
    State(app): State<AppState>,
    Json(payload): Json<CreateWorldPayload>,
) -> impl IntoResponse {

    let row = NewWorldRow {
           
        name: payload.name,
        description: payload.description,
        environment: payload.environment
            .unwrap_or_else(|| serde_json::to_value(earth_v0()).unwrap()),
        world_epoch: None,               // default epoch
    };

    match app.world_source.create_world(row).await {
        Ok(world_row) => {
            Json(WorldDto::from(world_row)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

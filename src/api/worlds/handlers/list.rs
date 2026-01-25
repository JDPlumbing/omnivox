use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use axum::http::StatusCode;
use serde_json::json;

use crate::shared::app_state::AppState;
use crate::api::worlds::dtos::world::WorldDto;

pub async fn list_worlds_handler(
    State(app): State<AppState>,
) -> impl IntoResponse {
    match app.world_source.list_worlds().await {
        Ok(rows) => {
            let worlds: Vec<WorldDto> =
                rows.iter().map(WorldDto::from_row).collect();

            Json(worlds).into_response()
        }
        Err(e) => {
            eprintln!("Error listing worlds: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "error listing worlds",
                    "details": format!("{e:?}")
                })),
            )
                .into_response()
        }
    }
}

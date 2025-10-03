use axum::{extract::Path, response::IntoResponse, Json};
use axum::http::StatusCode;

use crate::supabasic::worlds::{list_worlds, get_world_by_frame_id, create_world};
use crate::sim::world::{World, NewWorld};

/// DTO for returning worlds to the frontend
#[derive(serde::Serialize)]
pub struct WorldDto {
    pub frame_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl From<World> for WorldDto {
    fn from(w: World) -> Self {
        WorldDto {
            frame_id: w.frame_id,
            name: w.name,
            description: w.description,
        }
    }
}

/// GET /api/worlds
pub async fn list_worlds_handler() -> impl IntoResponse {
    match list_worlds().await {
        Ok(worlds) => {
            let dto_list: Vec<WorldDto> = worlds.into_iter().map(WorldDto::from).collect();
            Json(dto_list).into_response()
        }
        Err(e) => {
            eprintln!("Error listing worlds: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error listing worlds").into_response()
        }
    }
}

/// GET /api/worlds/:frame_id
pub async fn get_world_handler(Path(frame_id): Path<i64>) -> impl IntoResponse {
    match get_world_by_frame_id(frame_id).await {
        Ok(world) => Json(WorldDto::from(world)).into_response(),
        Err(e) => {
            eprintln!("Error fetching world {}: {:?}", frame_id, e);
            (StatusCode::NOT_FOUND, "world not found").into_response()
        }
    }
}

/// POST /api/worlds
pub async fn create_world_handler(Json(payload): Json<NewWorld>) -> impl IntoResponse {
    match create_world(&payload).await {
        Ok(world) => (StatusCode::CREATED, Json(WorldDto::from(world))).into_response(),
        Err(e) => {
            eprintln!("Error creating world: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error creating world").into_response()
        }
    }
}

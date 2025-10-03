use axum::{extract::Path, response::IntoResponse, Json};
use axum::http::StatusCode;
use crate::supabasic::orm::{insert, list};
use crate::sim::world::{World, NewWorld};



/// GET /api/worlds/{id}
pub async fn get_world(Path(frame_id): Path<i64>) -> impl IntoResponse {
    match list::<World>().await {
        Ok(worlds) => {
            if let Some(world) = worlds.into_iter().find(|w| w.frame_id == frame_id) {
                Json(world).into_response()
            } else {
                (StatusCode::NOT_FOUND, "not found").into_response()
            }
        }
        Err(e) => {
            eprintln!("Error listing worlds: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response()
        }
    }
}

/// POST /api/worlds
pub async fn create_new_world(Json(payload): Json<NewWorld>) -> impl IntoResponse {
    match insert::<NewWorld, World>(&payload).await {
        Ok(world) => (StatusCode::CREATED, Json(world)).into_response(),
        Err(e) => {
            eprintln!("Error creating world: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response()
        }
    }
}


/// GET /api/worlds
pub async fn list_worlds() -> impl IntoResponse {
    match list::<World>().await {
        Ok(worlds) => Json(worlds).into_response(),
        Err(e) => {
            eprintln!("Error listing worlds: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response()
        }
    }
}
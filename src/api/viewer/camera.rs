use axum::{
    Json,
    extract::State,
};
use axum::http::StatusCode;

use serde::Serialize;

use crate::shared::app_state::AppState;
use crate::core::id::UserId;

// REAL delta + movement functions
use crate::sim::systems::movement::camera_delta::CameraDelta;
use crate::sim::systems::movement::camera_movement::update_camera_from_delta;

use crate::sim::components::quaternion::QuaternionLocal;
use crate::core::uvoxid::UvoxId;

#[derive(Serialize)]
pub struct CameraStateResponse {
    pub pos: UvoxId,
    pub orient: QuaternionLocal,
}

pub async fn post_camera_delta(
    State(app): State<AppState>,
    Json(req): Json<CameraDelta>,
) -> Result<Json<CameraStateResponse>, StatusCode> {

    let user = app.user_owner_id
        .or(app.anon_owner_id)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut registry = app.viewer_registry.write().await;

    let cam = registry.cameras
        .entry(user)
        .or_insert_with(Default::default);

    // apply the delta using the real function
    update_camera_from_delta(&mut cam.pos, &mut cam.orient, req);

    Ok(Json(CameraStateResponse {
        pos: cam.pos,
        orient: cam.orient.clone(),
    }))
}

pub async fn get_camera_state(
    State(app): State<AppState>,
) -> Result<Json<CameraStateResponse>, StatusCode> {

    let user = app.user_owner_id
        .or(app.anon_owner_id)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let registry = app.viewer_registry.read().await;

    let cam = registry.cameras
        .get(&user)
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(CameraStateResponse {
        pos: cam.pos,
        orient: cam.orient.clone(),
    }))
}

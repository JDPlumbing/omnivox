use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use crate::{
    shared::app_state::AppState,
    shared::viewer_state::{ViewerRegistry, CameraState},
    core::id::UserId,
    sim::systems::movement::camera::CameraDelta,
    sim::systems::movement::camera::update_camera_from_delta,
};

use crate::sim::math::orientation::compute_global_orientation;

#[derive(Serialize)]
pub struct CameraStateResponse {
    pub pos: [f32; 3],
    pub orient: [f32; 4],
}

fn extract_user_id(headers: &HeaderMap) -> Result<UserId, StatusCode> {
    let raw = headers
        .get("x-user-id")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let s = raw.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(UserId::from_string(s))
}

/// POST /api/viewer/camera/delta
pub async fn post_camera_delta(
    State(app): State<AppState>,
    headers: HeaderMap,
    Json(delta): Json<CameraDelta>,
) -> impl IntoResponse {

    println!("================ CAMERA DELTA RECEIVED ================");
    println!("HEADERS: {:?}", headers);
    println!("DELTA BODY: {:?}", delta);

    


    let user_id = match extract_user_id(&headers) {
        Ok(id) => id,
        Err(code) => return code.into_response(),
    };

    // ⭐ async write lock
    let mut reg = app.viewer_registry.write().await;

    // Get or create camera slot
    let cam = reg.cameras
        .entry(user_id)
        .or_insert_with(CameraState::default);

    // Apply movement
    update_camera_from_delta(&mut cam.pos, &mut cam.orient, delta);

    StatusCode::OK.into_response()
}

/// GET /api/viewer/camera/state
pub async fn get_camera_state(
    State(app): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse 
{
    let user_id = match extract_user_id(&headers) {
        Ok(id) => id,
        Err(code) => return code.into_response(),
    };

    // ⭐ async read lock
    let reg = app.viewer_registry.read().await;


if let Some(cam) = reg.cameras.get(&user_id) {
    let q_global = compute_global_orientation(&cam.pos, &cam.orient);
    let q_arr = [q_global.x, q_global.y, q_global.z, q_global.w];

    return Json(CameraStateResponse {
        pos: cam.pos.to_vec3(),
        orient: q_arr,   // ✅ CORRECT
    }).into_response();
}


    Json(CameraStateResponse {
        pos: [0.0, 0.0, 2.0],
        orient: [0.0, 0.0, 0.0, 1.0],
    })
    .into_response()
}

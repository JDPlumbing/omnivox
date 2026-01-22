use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::shared::app_state::AppState;
use crate::core::observer::ObserverId;
use crate::core::id::WorldId;
use crate::core::tdt::SimTime;
use crate::core::world::presets::frames::frame_presets;
use crate::core::world::world_frame::WorldResolver;
use crate::core::physics::frames::local_tangent_frame;
use crate::core::physics::camera::{
    pose::CameraPose,
    basis::camera_basis_from_enu,
    project::project_world_dir_to_camera,
};
use crate::supabasic::worlds::WorldRow;

use crate::api::observers::dtos::camera::{
    CameraSkyResponse,
    CameraSkyObject,
};

use crate::core::math::vec3::normalize;

#[derive(Deserialize)]
pub struct CameraSkyQuery {
    pub time_ns: Option<String>,
    pub yaw_deg: Option<f64>,
    pub pitch_deg: Option<f64>,
}

pub async fn camera_sky_handler(
    State(app): State<AppState>,
    Path(observer_id): Path<u64>,
    Query(q): Query<CameraSkyQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    // -------------------------------------------------
    // Observer
    // -------------------------------------------------
    let observer = app.observers
        .read().await
        .get(&ObserverId(observer_id))
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    let time = q.time_ns
        .as_deref()
        .unwrap_or("0")
        .parse::<i128>()
        .map(SimTime)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let yaw = q.yaw_deg.unwrap_or(0.0).to_radians();
    let pitch = q.pitch_deg.unwrap_or(0.0).to_radians();

    // -------------------------------------------------
    // World + environment
    // -------------------------------------------------
    let world_row = WorldRow::get(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let env = world_row.environment
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let space = &env.space;

    // -------------------------------------------------
    // Resolver
    // -------------------------------------------------
    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    let earth = observer.world;
    let sun = WorldId(0);
    let moon = WorldId(2);

    // -------------------------------------------------
    // Local tangent frame
    // -------------------------------------------------
    let frame = local_tangent_frame(
        &resolver,
        earth,
        &observer.uvox,
        time,
        space,
    )
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    let pose = CameraPose {
        yaw_rad: yaw,
        pitch_rad: pitch,
    };

    let basis = camera_basis_from_enu(frame.enu, pose);
    let origin = frame.origin;

    // -------------------------------------------------
    // Sun
    // -------------------------------------------------
    let sun_pos = resolver.world_pose(sun, time).position_m;
    let sun_dir = normalize([
        sun_pos[0] - origin[0],
        sun_pos[1] - origin[1],
        sun_pos[2] - origin[2],
    ]);
    let sun_cam = project_world_dir_to_camera(basis, sun_dir);

    // -------------------------------------------------
    // Moon
    // -------------------------------------------------
    let moon_pos = resolver.world_pose(moon, time).position_m;
    let moon_dir = normalize([
        moon_pos[0] - origin[0],
        moon_pos[1] - origin[1],
        moon_pos[2] - origin[2],
    ]);
    let moon_cam = project_world_dir_to_camera(basis, moon_dir);

    // -------------------------------------------------
    // Response
    // -------------------------------------------------
    Ok(Json(CameraSkyResponse {
        observer_id,
        time_ns: time.0,
        sun: CameraSkyObject {
            x: sun_cam.x,
            y: sun_cam.y,
            z: sun_cam.z,
            visible: sun_cam.z > 0.0,
        },
        moon: CameraSkyObject {
            x: moon_cam.x,
            y: moon_cam.y,
            z: moon_cam.z,
            visible: moon_cam.z > 0.0,
        },
    }))
}

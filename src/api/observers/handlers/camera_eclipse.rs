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
    eclipse::{test_disk_overlap, EclipseType},
};
use crate::supabasic::worlds::WorldRow;
use crate::core::math::vec3::{normalize, magnitude};

use crate::api::observers::dtos::camera::CameraEclipseResponse;

#[derive(Deserialize)]
pub struct EclipseQuery {
    pub time_ns: Option<String>,
    pub yaw_deg: f64,
    pub pitch_deg: f64,
}

pub async fn camera_eclipse_handler(
    State(app): State<AppState>,
    Path(observer_id): Path<u64>,
    Query(q): Query<EclipseQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    // ----------------------------------
    // Observer
    // ----------------------------------
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

    // ----------------------------------
    // World + env
    // ----------------------------------
    let world_row = WorldRow::get(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let env = world_row.environment
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let space = &env.space;

    // ----------------------------------
    // Resolver
    // ----------------------------------
    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    // ----------------------------------
    // Local frame + camera
    // ----------------------------------
    let frame = local_tangent_frame(
        &resolver,
        observer.world,
        &observer.uvox,
        time,
        space,
    ).map_err(|_| StatusCode::BAD_REQUEST)?;

    let basis = camera_basis_from_enu(
        frame.enu,
        CameraPose {
            yaw_rad: q.yaw_deg.to_radians(),
            pitch_rad: q.pitch_deg.to_radians(),
        },
    );

    let origin = frame.origin;

   // ----------------------------------
// Bodies
// ----------------------------------
let sun = WorldId(0);
let moon = WorldId(2);

// Fetch frames ONCE
let sun_frame = resolver.frames.get(&sun).unwrap();
let moon_frame = resolver.frames.get(&moon).unwrap();

// Helper: camera vector + distance
let body_cam_and_dist = |body: WorldId| {
    let pos = resolver.world_pose(body, time).position_m;
    let v = [
        pos[0] - origin[0],
        pos[1] - origin[1],
        pos[2] - origin[2],
    ];
    let distance = magnitude(v);
    let dir = normalize(v);
    let cam = project_world_dir_to_camera(basis, dir);
    (cam, distance)
};

// Compute once
let (sun_cam, sun_dist) = body_cam_and_dist(sun);
let (moon_cam, moon_dist) = body_cam_and_dist(moon);

// Angular radii
let sun_radius_rad = {
    let r = sun_frame.physical_radius_m
        .ok_or(StatusCode::BAD_REQUEST)?;
    (r / sun_dist).atan()
};

let moon_radius_rad = {
    let r = moon_frame.physical_radius_m
        .ok_or(StatusCode::BAD_REQUEST)?;
    (r / moon_dist).atan()
};


// If either body is behind the camera, no eclipse is visible
if sun_cam.z <= 0.0 || moon_cam.z <= 0.0 {
    return Ok(Json(CameraEclipseResponse {
        observer_id,
        time_ns: time.0,
        eclipse: "none".to_string(),
        center_separation_rad: f64::NAN,
        sun_radius_rad,
        moon_radius_rad,
    }));
}

    // ----------------------------------
    // Eclipse test
    // ----------------------------------
    let result = test_disk_overlap(
        sun_cam,
        sun_radius_rad,
        moon_cam,
        moon_radius_rad,
    );

    let eclipse_str = match result.eclipse {
        EclipseType::None => "none",
        EclipseType::Partial => "partial",
        EclipseType::Annular => "annular",
        EclipseType::Total => "total",
    }.to_string();

    Ok(Json(CameraEclipseResponse {
        observer_id,
        time_ns: time.0,
        eclipse: match result.eclipse {
            EclipseType::None => "none",
            EclipseType::Partial => "partial",
            EclipseType::Annular => "annular",
            EclipseType::Total => "total",
        }.to_string(),
        center_separation_rad: result.center_separation_rad,
        sun_radius_rad,
        moon_radius_rad,
    }))

}

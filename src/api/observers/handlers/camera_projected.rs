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
    project::{project_world_dir_to_camera, CameraProjection},
};
use crate::supabasic::worlds::WorldRow;
use crate::core::math::vec3::normalize;

use crate::api::observers::dtos::camera::{
    CameraSkyProjectedResponse,
    CameraBodyProjection,
};

#[derive(Deserialize)]
pub struct CameraProjectedQuery {
    pub time_ns: Option<String>,
    pub yaw_deg: f64,
    pub pitch_deg: f64,
    pub fov_y_deg: Option<f64>,
    pub aspect: Option<f64>,
}

pub async fn camera_sky_projected_handler(
    State(app): State<AppState>,
    Path(observer_id): Path<u64>,
    Query(q): Query<CameraProjectedQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    // Observer
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

    // World + env
    let world_row = WorldRow::get(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let env = world_row.environment
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let space = &env.space;

    // Resolver
    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    // Local tangent frame
    let frame = local_tangent_frame(
        &resolver,
        observer.world,
        &observer.uvox,
        time,
        space,
    ).map_err(|_| StatusCode::BAD_REQUEST)?;

    // Camera
    let basis = camera_basis_from_enu(
        frame.enu,
        CameraPose {
            yaw_rad: q.yaw_deg.to_radians(),
            pitch_rad: q.pitch_deg.to_radians(),
        },
    );

    let projection = CameraProjection {
        fov_y_rad: q.fov_y_deg.unwrap_or(60.0).to_radians(),
        aspect: q.aspect.unwrap_or(16.0 / 9.0),
    };

    let earth = observer.world;
    let sun = WorldId(0);
    let moon = WorldId(2);

    let origin = resolver.world_anchor_point(
        earth,
        &observer.uvox,
        time,
        space,
    ).map_err(|_| StatusCode::BAD_REQUEST)?;

    let project_body = |body: WorldId| {
        let pos = resolver.world_pose(body, time).position_m;

        let v = [
            pos[0] - origin[0],
            pos[1] - origin[1],
            pos[2] - origin[2],
        ];

        let distance = (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt();
        let dir = normalize(v);

        let cam = project_world_dir_to_camera(basis, dir);

        let angular_radius_rad =
            (body_radius_m(body) / distance).atan();

        CameraBodyProjection {
            visible: projection.is_visible(cam),
            ndc: projection.project_to_ndc(cam),
            angular_radius_rad,
        }
    };


    Ok(Json(CameraSkyProjectedResponse {
        observer_id,
        time_ns: time.0,
        sun: project_body(sun),
        moon: project_body(moon),
    }))
}


//helpers

fn body_radius_m(body: WorldId) -> f64 {
    match body {
        WorldId(0) => 696_340_000.0, // Sun
        WorldId(2) => 1_737_400.0,   // Moon
        _ => 0.0,
    }
}

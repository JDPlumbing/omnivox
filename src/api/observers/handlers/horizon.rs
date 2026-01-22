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
use crate::core::tdt::{SimTime, SimDuration};
use crate::core::world::presets::frames::frame_presets;
use crate::core::world::world_frame::WorldResolver;
use crate::core::physics::frames::local_tangent_frame;
use crate::core::physics::camera::*;
use crate::supabasic::worlds::WorldRow;

use crate::api::observers::dtos::horizon::*;
use crate::core::math::vec3::{magnitude, normalize};

#[derive(Deserialize)]
pub struct HorizonQuery {
    pub start_time_ns: String,
    pub end_time_ns: String,
    pub step_ns: Option<String>,
    pub yaw_deg: Option<f64>,
    pub pitch_deg: Option<f64>,
}

pub async fn camera_horizon_handler(
    State(app): State<AppState>,
    Path(observer_id): Path<u64>,
    Query(q): Query<HorizonQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let observer = app.observers
        .read().await
        .get(&ObserverId(observer_id))
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    let start_time = SimTime(
        q.start_time_ns.parse::<i128>().map_err(|_| StatusCode::BAD_REQUEST)?
    );
    let end_time = SimTime(
        q.end_time_ns.parse::<i128>().map_err(|_| StatusCode::BAD_REQUEST)?
    );

    let step = SimDuration::from_ns(
        q.step_ns
            .as_deref()
            .unwrap_or("300000000000") // 5 minutes
            .parse::<i128>()
            .map_err(|_| StatusCode::BAD_REQUEST)?
    );

    let yaw = q.yaw_deg.unwrap_or(0.0).to_radians();
    let pitch = q.pitch_deg.unwrap_or(0.0).to_radians();

    let world_row = WorldRow::get(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let env = world_row.environment
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let space = &env.space;

    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    let earth = observer.world;
    let sun = WorldId(0);
    let moon = WorldId(2);

    let pose = CameraPose { yaw_rad: yaw, pitch_rad: pitch };

    let mut events = Vec::new();

    let mut prev_sun_z: Option<f64> = None;
    let mut prev_moon_z: Option<f64> = None;

    let mut t = start_time;

    while t.0 <= end_time.0 {
        let frame = local_tangent_frame(
            &resolver,
            earth,
            &observer.uvox,
            t,
            space,
        ).map_err(|_| StatusCode::BAD_REQUEST)?;

        let basis = camera_basis_from_enu(frame.enu, pose);

        let obs_pos = frame.origin;

        // -------- Sun --------
        let sun_pos = resolver.world_pose(sun, t).position_m;
        let sun_dir = normalize([
            sun_pos[0] - obs_pos[0],
            sun_pos[1] - obs_pos[1],
            sun_pos[2] - obs_pos[2],
        ]);
        let sun_cam = project_world_dir_to_camera(basis, sun_dir);

        if let Some(prev) = prev_sun_z {
            if prev <= 0.0 && sun_cam.z > 0.0 {
                events.push(HorizonEvent {
                    body: "sun".into(),
                    time_ns: t.0,
                    event: "rise".into(),
                });
            }
            if prev >= 0.0 && sun_cam.z < 0.0 {
                events.push(HorizonEvent {
                    body: "sun".into(),
                    time_ns: t.0,
                    event: "set".into(),
                });
            }
        }
        prev_sun_z = Some(sun_cam.z);

        // -------- Moon --------
        let moon_pos = resolver.world_pose(moon, t).position_m;
        let moon_dir = normalize([
            moon_pos[0] - obs_pos[0],
            moon_pos[1] - obs_pos[1],
            moon_pos[2] - obs_pos[2],
        ]);
        let moon_cam = project_world_dir_to_camera(basis, moon_dir);

        if let Some(prev) = prev_moon_z {
            if prev <= 0.0 && moon_cam.z > 0.0 {
                events.push(HorizonEvent {
                    body: "moon".into(),
                    time_ns: t.0,
                    event: "rise".into(),
                });
            }
            if prev >= 0.0 && moon_cam.z < 0.0 {
                events.push(HorizonEvent {
                    body: "moon".into(),
                    time_ns: t.0,
                    event: "set".into(),
                });
            }
        }
        prev_moon_z = Some(moon_cam.z);

        t = SimTime(t.0 + step.0);
    }

    Ok(Json(CameraHorizonResponse {
        observer_id,
        events,
    }))
}

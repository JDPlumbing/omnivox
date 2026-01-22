use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::shared::app_state::AppState;
use crate::core::observer::ObserverId;
use crate::core::tdt::{SimTime, SimDuration};
use crate::core::physics::camera::{
    pose::CameraPose,
    eclipse_timeline::compute_eclipse_timeline,
};
use crate::core::world::presets::frames::frame_presets;
use crate::core::world::world_frame::WorldResolver;
use crate::supabasic::worlds::WorldRow;

use crate::api::observers::dtos::camera_eclipse::{
    CameraEclipseTimelineResponse,
    CameraEclipseEvent,
};

#[derive(Deserialize)]
pub struct EclipseTimelineQuery {
    pub time_start_ns: String,
    pub time_end_ns: String,
    pub step_ns: String,
    pub yaw_deg: f64,
    pub pitch_deg: f64,
}


pub async fn camera_eclipse_timeline_handler(
    State(app): State<AppState>,
    Path(observer_id): Path<u64>,
    Query(q): Query<EclipseTimelineQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let observer = app.observers
        .read().await
        .get(&ObserverId(observer_id))
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

        let start = SimTime(
            q.time_start_ns.parse::<i128>()
                .map_err(|_| StatusCode::BAD_REQUEST)?
        );

        let end = SimTime(
            q.time_end_ns.parse::<i128>()
                .map_err(|_| StatusCode::BAD_REQUEST)?
        );

        let step = SimDuration(
            q.step_ns.parse::<i128>()
                .map_err(|_| StatusCode::BAD_REQUEST)?
        );


    let world_row = WorldRow::get(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let env = world_row.environment
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    let timeline = compute_eclipse_timeline(
        &resolver,
        observer.world,
        &observer.uvox,
        &env.space,
        CameraPose {
            yaw_rad: q.yaw_deg.to_radians(),
            pitch_rad: q.pitch_deg.to_radians(),
        },
        start,
        end,
        step,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CameraEclipseTimelineResponse {
        observer_id,
        events: timeline.events.into_iter().map(|e| CameraEclipseEvent {
            time_ns: e.time.0,
            state: format!("{:?}", e.state).to_lowercase(),
        }).collect(),
    }))
}

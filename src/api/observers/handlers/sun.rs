use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::shared::app_state::AppState;
use crate::api::observers::dtos::*;
use crate::core::observer::{Observer, ObserverId};
use crate::core::uvoxid::UvoxId;
use crate::core::tdt::{SimTime, sim_duration::SimDuration};
use crate::engine::world::loader::load_world;
use crate::core::WorldId;
use crate::core::world::presets::frames::frame_presets;
use crate::core::world::world_frame::WorldResolver;
use crate::core::physics::environmental_snapshot::{
    EnvironmentalSnapshot,
    sample_environmental_snapshot,
};

static OBSERVER_SEQ: AtomicU64 = AtomicU64::new(1);
use crate::core::physics::frames::local_tangent::{
    local_tangent_frame,
};
use std::f64::consts::PI;
use crate::core::physics::illumination::sun_direction_world;
use crate::api::observers::handlers::ObserverFrameQuery;

pub async fn get_observer_sun_angles(
    State(app): State<AppState>,
    Path(id): Path<u64>,
    Query(q): Query<ObserverFrameQuery>, // reuse time_ns
) -> Result<impl IntoResponse, StatusCode> {
    let observer = app.observers
        .read().await
        .get(&ObserverId(id))
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;
    let time = q.time_ns
        .as_deref()
        .unwrap_or("0")
        .parse::<i128>()
        .map(SimTime)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let world = load_world(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };
    let space = &world.environment.space;
    let frame = local_tangent_frame(
        &resolver,
        observer.world,
        &observer.uvox,
        time,
        space,
    ).map_err(|_| StatusCode::BAD_REQUEST)?;
   
    let obs_pos = frame.origin;

    let sun = WorldId(0);

    let sun_dir_world = sun_direction_world(
        &resolver,
        observer.world,
        &observer.uvox,
        sun,
        time,
        &world.environment.space,
    ).map_err(|_| StatusCode::BAD_REQUEST)?;

    let [e, n, u] = frame.enu.project(sun_dir_world);

    let elevation = u.asin();

    let mut azimuth = e.atan2(n);
    if azimuth < 0.0 {
        azimuth += 2.0 * std::f64::consts::PI;
    }


    Ok(Json(SunAnglesResponse {
        observer_id: observer.id.0,
        time_ns: time.0,
        azimuth_deg: azimuth.to_degrees(),
        elevation_deg: elevation.to_degrees(),
    }))
}
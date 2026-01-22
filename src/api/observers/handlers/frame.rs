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


#[derive(Deserialize)]
pub struct ObserverFrameQuery {
    pub time_ns: Option<String>,
}


/// ------------------------------------------------------------
/// Get observer local tangent frame (ENU)
/// ------------------------------------------------------------
pub async fn get_observer_frame(
    State(app): State<AppState>,
    Path(id): Path<u64>,
    Query(q): Query<ObserverFrameQuery>,
) -> Result<impl IntoResponse, StatusCode> {

    // 1. Fetch observer
    let observer = app.observers
        .read().await
        .get(&ObserverId(id))
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    // 2. Parse time (default = 0)
    let time = q.time_ns
        .as_deref()
        .unwrap_or("0")
        .parse::<i128>()
        .map(SimTime)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // 3. Load world
    let world = load_world(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let space = &world.environment.space;

    // 4. Build resolver
    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    // 5. Compute local tangent frame (THIS IS THE CORE)
    let frame = local_tangent_frame(
        &resolver,
        observer.world,
        &observer.uvox,
        time,
        space,
    ).map_err(|_| StatusCode::BAD_REQUEST)?;

    // 6. Serialize
    Ok(Json(ObserverFrameResponse {
        observer_id: observer.id.0,
        time_ns: time.0,
        origin: frame.origin,
        enu: EnuFrameDto {
            east: frame.enu.east,
            north: frame.enu.north,
            up: frame.enu.up,
        },
    }))
}
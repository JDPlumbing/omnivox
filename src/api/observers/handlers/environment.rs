
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

/// ------------------------------------------------------------
/// Sample observer environment (single snapshot)
/// ------------------------------------------------------------
#[derive(Deserialize)]
pub struct ObserverSampleQuery {
    pub time_ns: Option<String>,
}

pub async fn sample_observer_environment(
    State(app): State<AppState>,
    Path(id): Path<u64>,
    Query(q): Query<ObserverSampleQuery>,
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

    let snapshot = sample_environmental_snapshot(
        &resolver,
        observer.world,
        &observer.uvox,
        time,
        &world.environment,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(snapshot))
}


pub async fn environmental_curve_handler(
    State(app): State<AppState>,
    Path(observer_id): Path<u64>,
    Query(q): Query<EnvironmentalCurveQuery>,
) -> Result<impl IntoResponse, StatusCode> {

        println!("🔥 environmental_curve_handler HIT for observer {}", observer_id);
    let observer = app.observers
        .read().await
        .get(&ObserverId(observer_id))
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    let start_time = SimTime(
        q.start_time_ns
            .parse::<i128>()
            .map_err(|_| StatusCode::BAD_REQUEST)?,
    );

    let step_ns = q.step_ns
        .as_deref()
        .unwrap_or("3600000000000") // 1 hour
        .parse::<i128>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let step = SimDuration::from_ns(step_ns);
    let count = q.samples.unwrap_or(24);

    let world = load_world(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    let mut out = Vec::with_capacity(count);
    let mut t = start_time;

    for _ in 0..count {
        let snapshot = sample_environmental_snapshot(
            &resolver,
            observer.world,
            &observer.uvox,
            t,
            &world.environment,
        ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        out.push(EnvironmentalSample {
            time_ns: t.0,
            snapshot,
        });

        t = SimTime(t.0 + step.0);
    }

    Ok(Json(EnvironmentalCurveResponse {
        observer_id,
        samples: out,
    }))
}



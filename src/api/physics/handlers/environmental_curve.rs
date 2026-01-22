use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::core::tdt::{SimTime, sim_duration::SimDuration};
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_frame::WorldResolver;
use crate::core::world::presets::frames::frame_presets;
use crate::engine::world::loader::load_world;

use crate::core::physics::environmental_snapshot::{
    EnvironmentalSnapshot,
    sample_environmental_snapshot,
};

#[derive(Deserialize)]
pub struct EnvironmentalSnapshotCurveQuery {
    pub world: WorldId,
    pub uvox: String,
    pub start_time_ns: String,
    pub duration_days: Option<u64>,
    pub step_seconds: Option<u64>,
}

#[derive(Serialize)]
pub struct EnvironmentalSnapshotSample {
    pub time_ns: i128,
    pub snapshot: EnvironmentalSnapshot,
}

#[derive(Serialize)]
pub struct EnvironmentalSnapshotCurveResponse {
    pub step_seconds: u64,
    pub samples: Vec<EnvironmentalSnapshotSample>,
}

pub async fn environmental_snapshot_curve_handler(
    State(app): State<AppState>,
    Query(q): Query<EnvironmentalSnapshotCurveQuery>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    // -----------------------------
    // Parse inputs
    // -----------------------------
    let uvox = UvoxId::from_hex(&q.uvox)
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let start_time = SimTime(
        q.start_time_ns
            .parse::<i128>()
            .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?,
    );

    let duration_days = q.duration_days.unwrap_or(1);
    let step_seconds = q.step_seconds.unwrap_or(3600);

    let duration = SimDuration::days(duration_days as i64);
    let step = SimDuration::seconds(step_seconds as i64);

    // -----------------------------
    // Load world
    // -----------------------------
    let world = load_world(&app.supa, q.world)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    // -----------------------------
    // Resolver
    // -----------------------------
    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    let env = &world.environment;

    // -----------------------------
    // Sample curve
    // -----------------------------
    let mut samples = Vec::new();
    let mut t = start_time;

    while t.0 <= start_time.0 + duration.0 {
        let snapshot = sample_environmental_snapshot(
            &resolver,
            q.world,
            &uvox,
            t,
            env,
        )
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        samples.push(EnvironmentalSnapshotSample {
            time_ns: t.0,
            snapshot,
        });

        t = SimTime(t.0 + step.0);
    }

    Ok(Json(EnvironmentalSnapshotCurveResponse {
        step_seconds,
        samples,
    }))
}

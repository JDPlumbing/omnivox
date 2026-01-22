use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::core::tdt::{SimTime, SimDuration};
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_frame::WorldResolver;
use crate::core::world::presets::frames::frame_presets;
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::physics::insolation::seasonal_insolation_curve;
use crate::supabasic::worlds::WorldRow;

#[derive(Deserialize)]
pub struct InsolationCurveQuery {
    pub world: WorldId,
    pub uvox: String,
    pub start_time_ns: String,
    pub step_days: Option<u64>,
    pub day_samples: Option<usize>,
}

#[derive(Serialize)]
pub struct InsolationSample {
    pub time_ns: i128,
    pub insolation: f64,
}

pub async fn insolation_curve_handler(
    State(app): State<AppState>,
    Query(q): Query<InsolationCurveQuery>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let uvox = UvoxId::from_hex(&q.uvox)
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let start_time = SimTime(
        q.start_time_ns
            .parse::<i128>()
            .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?,
    );

    let step_days = q.step_days.unwrap_or(10) as i64;
    let step = SimDuration::days(step_days);

    let day_samples = q.day_samples.unwrap_or(144);

    let world_row = WorldRow::get(&app.supa, q.world)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    let env = world_row
        .environment
        .as_ref()
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let space: &WorldSpace = &env.space;

    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    let curve = seasonal_insolation_curve(
        &resolver,
        q.world,
        &uvox,
        WorldId(0), // Sun
        start_time,
        space,
        step,
        day_samples,
    )
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let samples = curve
        .into_iter()
        .map(|(t, i)| InsolationSample {
            time_ns: t.0,
            insolation: i,
        })
        .collect::<Vec<_>>();

    Ok(Json(samples))
}

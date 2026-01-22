use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::core::tdt::SimTime;
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_frame::WorldResolver;
use crate::core::world::presets::frames::frame_presets;
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::physics::insolation::daily_insolation;
use crate::supabasic::worlds::WorldRow;

#[derive(Deserialize)]
pub struct DailyInsolationQuery {
    pub world: WorldId,
    pub uvox: String,
    pub time_ns: String,
    pub samples: Option<usize>,
}

#[derive(Serialize)]
pub struct DailyInsolationResponse {
    pub insolation: f64,
}

pub async fn daily_insolation_handler(
    State(app): State<AppState>,
    Query(q): Query<DailyInsolationQuery>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let uvox = UvoxId::from_hex(&q.uvox)
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let time = SimTime(
        q.time_ns
            .parse::<i128>()
            .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?,
    );

    let samples = q.samples.unwrap_or(144);

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

    let insolation = daily_insolation(
        &resolver,
        q.world,
        &uvox,
        WorldId(0), // Sun
        time,
        space,
        samples,
    )
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(DailyInsolationResponse { insolation }))
}

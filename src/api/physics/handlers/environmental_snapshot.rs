use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::core::tdt::SimTime;
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_frame::WorldResolver;
use crate::core::world::presets::frames::frame_presets;
use crate::engine::world::loader::load_world;

use crate::core::physics::environmental_snapshot::{
    EnvironmentalSnapshot,
    sample_environmental_snapshot,
};

#[derive(Deserialize)]
pub struct EnvironmentalSnapshotQuery {
    pub world: WorldId,
    pub uvox: String,
    pub time_ns: Option<String>,
}

pub async fn environmental_snapshot_handler(
    State(app): State<AppState>,
    Query(q): Query<EnvironmentalSnapshotQuery>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let uvox = UvoxId::from_hex(&q.uvox)
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let time = SimTime(
        q.time_ns
            .as_deref()
            .unwrap_or("0")
            .parse::<i128>()
            .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?,
    );

    let world = load_world(&app.supa, q.world)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    let snapshot: EnvironmentalSnapshot = sample_environmental_snapshot(
        &resolver,
        q.world,
        &uvox,
        time,
        &world.environment,
    )
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(snapshot))
}

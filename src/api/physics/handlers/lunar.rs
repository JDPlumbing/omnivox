// api/physics/handlers.rs

use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::core::tdt::SimTime;
use crate::core::world::world_frame::WorldResolver;
use crate::core::world::presets::frames::frame_presets;
use crate::core::physics::lunar_phase::{lunar_phase, LunarPhase};

#[derive(Deserialize)]
pub struct PhaseQuery {
    pub time_ns: String,
}

pub async fn lunar_phase_handler(
    State(_app): State<AppState>,
    Query(q): Query<PhaseQuery>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let time = SimTime(
        q.time_ns
            .parse::<i128>()
            .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?,
    );

    // Build resolver
    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    // World IDs (canonical)
    let sun = WorldId(0);
    let earth = WorldId(1);
    let moon = WorldId(2);

    let phase: LunarPhase = lunar_phase(
        &resolver,
        moon,
        earth,
        sun,
        time,
    );

    Ok(Json(phase))
}

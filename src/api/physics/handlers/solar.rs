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
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::physics::illumination::solar_illumination;
use crate::supabasic::worlds::WorldRow;

#[derive(Deserialize)]
pub struct IlluminationQuery {
    pub world: WorldId,
    pub uvox: String,
    pub time_ns: String,
}

pub async fn solar_illumination_handler(
    State(app): State<AppState>,
    Query(q): Query<IlluminationQuery>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let uvox = UvoxId::from_hex(&q.uvox)
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let time = SimTime(
        q.time_ns
            .parse::<i128>()
            .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?,
    );

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

    let sun = WorldId(0);

    let illumination = solar_illumination(
        &resolver,
        q.world,
        &uvox,
        sun,
        time,
        space,
    )
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "illumination": illumination
    })))
}

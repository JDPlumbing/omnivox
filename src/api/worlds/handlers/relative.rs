use axum::{
    extract::{State, Query, Path},
    response::IntoResponse,
    Json,
};
use std::collections::HashMap;
use anyhow::{anyhow, Result};
use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::core::tdt::SimTime;
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_frame::{WorldFrame, WorldResolver};
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::world::world_relative::world_to_world_vector;
use crate::supabasic::worlds::WorldRow;
// 👇 YOU decide how frames are loaded (DB, static, preset)
use crate::core::world::presets::frames::frame_presets;


#[derive(serde::Deserialize)]
pub struct RelativeQuery {
    pub from_uvox: String,
    pub time_ns: String,
}

pub async fn world_relative_handler(
    State(app): State<AppState>,
    Path((from_world, to_world)): Path<(WorldId, WorldId)>,
    Query(q): Query<RelativeQuery>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {

    // --------------------------------------------------
    // 1. Parse inputs
    // --------------------------------------------------

    let from_uvox = UvoxId::from_hex(&q.from_uvox)
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let time = SimTime(
        q.time_ns
            .parse::<i128>()
            .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?
    );

    // --------------------------------------------------
    // 2. Load frames
    // --------------------------------------------------

    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    // --------------------------------------------------
    // 3. Load WorldSpace
    // --------------------------------------------------

    let world_row = WorldRow::get(&app.supa, from_world)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    let env_desc = world_row.environment
        .as_ref()
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let space = &env_desc.space;

    // --------------------------------------------------
    // 4. Compute relative vector
    // --------------------------------------------------

    let vec = world_to_world_vector(
        &resolver,
        from_world,
        &from_uvox,
        to_world,
        time,
        space,
    );

    Ok(Json(vec))
}

#[derive(serde::Deserialize)]
pub struct TimeQuery {
    pub time_ns: String,
}

pub async fn world_origin_relative_handler(
    State(app): State<AppState>,
    Path((from_world, to_world)): Path<(WorldId, WorldId)>,
    Query(q): Query<TimeQuery>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let time = SimTime(
        q.time_ns
            .parse::<i128>()
            .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?
    );

    // Load frames
    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    // Compute origin → origin vector
    let from_pose = resolver.world_pose(from_world, time);
    let to_pose = resolver.world_pose(to_world, time);

    let vec = [
        to_pose.position_m[0] - from_pose.position_m[0],
        to_pose.position_m[1] - from_pose.position_m[1],
        to_pose.position_m[2] - from_pose.position_m[2],
    ];

    Ok(Json(vec))
}

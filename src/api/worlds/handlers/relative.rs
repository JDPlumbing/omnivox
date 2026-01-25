use axum::{
    extract::{State, Query, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::str::FromStr;
use std::sync::Arc;

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::core::tdt::sim_time::SimTime;
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_relative::world_to_world_vector;
use crate::core::world::world_frame::WorldResolver;
use crate::core::world::presets::frames::frame_presets;
use crate::engine::world::state::WorldState;

#[derive(Deserialize)]
pub struct RelativeQuery {
    pub from_uvox: String,
    pub time_ns: i128,
}

pub async fn world_relative_handler(
    State(app): State<AppState>,
    Path((from_world, to_world)): Path<(WorldId, WorldId)>,
    Query(q): Query<RelativeQuery>,
) -> impl IntoResponse {
    // --------------------------------------------------
    // 1. Parse inputs
    // --------------------------------------------------

    let from_uvox = match UvoxId::from_str(&q.from_uvox) {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "invalid uvox id"
                })),
            )
                .into_response();
        }
    };

    let time = SimTime::from_ns(q.time_ns);

    // --------------------------------------------------
    // 2. Get world (runtime → source fallback)
    // --------------------------------------------------

    let cached: Option<Arc<WorldState>> = {
        let worlds = app.worlds.read().await;
        worlds.get(&from_world).cloned()
    };

    let world: Arc<WorldState> = match cached {
        Some(w) => w,
        None => {
            match app.world_source.load_world(from_world).await {
                Ok(loaded) => {
                    let arc = Arc::new(loaded);
                    app.worlds
                        .write()
                        .await
                        .insert(from_world, arc.clone());
                    arc
                }
                Err(e) => {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(serde_json::json!({
                            "error": "world not found",
                            "details": e.to_string()
                        })),
                    )
                        .into_response();
                }
            }
        }
    };

    // --------------------------------------------------
    // 3. Load frames & resolver
    // --------------------------------------------------

    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    // --------------------------------------------------
    // 4. Compute relative vector
    // --------------------------------------------------

    let space = &world.environment.space;

    let vec = world_to_world_vector(
        &resolver,
        from_world,
        &from_uvox,
        to_world,
        time,
        space,
    );

    Json(vec).into_response()
}

#[derive(Deserialize)]
pub struct TimeQuery {
    pub time_ns: i128,
}

pub async fn world_origin_relative_handler(
    State(_app): State<AppState>,
    Path((from_world, to_world)): Path<(WorldId, WorldId)>,
    Query(q): Query<TimeQuery>,
) -> impl IntoResponse {
    let time = SimTime::from_ns(q.time_ns);

    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    let from_pose = resolver.world_pose(from_world, time);
    let to_pose = resolver.world_pose(to_world, time);

    let vec = [
        to_pose.position_m[0] - from_pose.position_m[0],
        to_pose.position_m[1] - from_pose.position_m[1],
        to_pose.position_m[2] - from_pose.position_m[2],
    ];

    Json(vec).into_response()
}

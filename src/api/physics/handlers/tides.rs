use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Serialize, Deserialize};

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::core::tdt::SimTime;
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_frame::WorldResolver;
use crate::core::world::presets::frames::frame_presets;
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::physics::tides::{
    tidal_potential,
    tidal_acceleration,
};
use crate::core::physics::frames::local_tangent_frame;
use crate::supabasic::worlds::WorldRow;

#[derive(Deserialize)]
pub struct TidesQuery {
    pub world: WorldId,
    pub uvox: String,
    pub time_ns: String,
}

// ------------------------------------------------------------
// Tidal potential handler
// ------------------------------------------------------------

pub async fn tidal_potential_handler(
    State(app): State<AppState>,
    Query(q): Query<TidesQuery>,
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

    let (lunar, solar, total) = tidal_potential(
        &resolver,
        q.world,
        &uvox,
        time,
        space,
    )
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "lunar": lunar,
        "solar": solar,
        "total": total
    })))
}

// ------------------------------------------------------------
// Tidal acceleration handler
// ------------------------------------------------------------

#[derive(Serialize)]
pub struct TidalAccelVector {
    pub world: [f64; 3],
    pub enu: [f64; 3],
    pub lateral_magnitude: f64,
}

#[derive(Serialize)]
pub struct TidalAccelerationResponse {
    pub moon: TidalAccelVector,
    pub solar: TidalAccelVector,
    pub total: TidalAccelVector,
}

pub async fn tidal_acceleration_handler(
    State(app): State<AppState>,
    Query(q): Query<TidesQuery>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let uvox = UvoxId::from_hex(&q.uvox)
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let time = SimTime(
        q.time_ns
            .parse::<i128>()
            .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?,
    );

    let world = q.world;

    let world_row = WorldRow::get(&app.supa, world)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    let env = world_row
        .environment
        .as_ref()
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let space = &env.space;

    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    let (a_moon, a_solar, a_total) =
        tidal_acceleration(&resolver, world, &uvox, time, space)
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let frame = local_tangent_frame(
        &resolver,
        world,
        &uvox,
        time,
        space,
    )
    .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    let enu = frame.enu;

    let to_vec = |a: [f64; 3]| {
        let enu_vec = enu.project(a);
        let lateral =
            (enu_vec[0] * enu_vec[0] + enu_vec[1] * enu_vec[1]).sqrt();

        TidalAccelVector {
            world: a,
            enu: enu_vec,
            lateral_magnitude: lateral,
        }
    };

    Ok(Json(TidalAccelerationResponse {
        moon: to_vec(a_moon),
        solar: to_vec(a_solar),
        total: to_vec(a_total),
    }))
}

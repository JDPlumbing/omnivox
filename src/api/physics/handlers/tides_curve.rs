use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Serialize, Deserialize};

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::core::tdt::{SimTime, SimDuration};
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_frame::WorldResolver;
use crate::core::world::presets::frames::frame_presets;
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::physics::tides::tidal_acceleration;
use crate::supabasic::worlds::WorldRow;

#[derive(Deserialize)]
pub struct TidesCurveQuery {
    pub world: WorldId,
    pub uvox: String,
    pub start_time_ns: String,
}

#[derive(Serialize)]
pub struct TideSample {
    pub time_ns: i128,
    pub lunar: f64,
    pub solar: f64,
    pub total: f64,
}

#[derive(Serialize)]
pub struct TideCurveResponse {
    pub step_seconds: i64,
    pub samples: Vec<TideSample>,
}

pub async fn tides_curve_handler(
    State(app): State<AppState>,
    Query(q): Query<TidesCurveQuery>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    // ----------------------------
    // Parse inputs
    // ----------------------------

    let uvox = UvoxId::from_hex(&q.uvox)
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let start_time = SimTime(
        q.start_time_ns
            .parse::<i128>()
            .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?,
    );

    let world = q.world;

    // ----------------------------
    // Load environment
    // ----------------------------

    let world_row = WorldRow::get(&app.supa, world)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    let env = world_row
        .environment
        .as_ref()
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let space: &WorldSpace = &env.space;

    // ----------------------------
    // Resolver
    // ----------------------------

    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    // ----------------------------
    // Sampling configuration
    // ----------------------------

    let step = SimDuration::minutes(30);
    let window = SimDuration::hours(24);

    let mut samples = Vec::new();
    let mut t = start_time;

    fn lateral_mag(v: [f64; 3]) -> f64 {
        (v[0] * v[0] + v[1] * v[1]).sqrt()
    }

    while t.0 < start_time.0 + window.0 {
        let (lunar_vec, solar_vec, total_vec) =
            tidal_acceleration(&resolver, world, &uvox, t, space)
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        samples.push(TideSample {
            time_ns: t.0,
            lunar: lateral_mag(lunar_vec),
            solar: lateral_mag(solar_vec),
            total: lateral_mag(total_vec),
        });

        t = SimTime(t.0 + step.0);
    }

    Ok(Json(TideCurveResponse {
        step_seconds: 1800,
        samples,
    }))
}

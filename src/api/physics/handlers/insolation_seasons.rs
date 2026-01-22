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
use crate::supabasic::worlds::WorldRow;

use crate::core::physics::insolation::daily_insolation;
use crate::core::physics::insolation_seasons::{
    InsolationSample,
    detect_insolation_seasons,
};

/// --------------------
/// Query
/// --------------------

#[derive(Deserialize)]
pub struct InsolationSeasonsQuery {
    pub world: WorldId,
    pub uvox: String,
    pub start_time_ns: String,
}

/// --------------------
/// Output structs
/// --------------------

#[derive(Serialize)]
pub struct InsolationSeasonOut {
    pub time_ns: i128,
    pub insolation: f64,
}

#[derive(Serialize)]
pub struct InsolationSeasonsResponse {
    pub summer_solstice: Option<InsolationSeasonOut>,
    pub winter_solstice: Option<InsolationSeasonOut>,
    pub vernal_equinox: Option<InsolationSeasonOut>,
    pub autumnal_equinox: Option<InsolationSeasonOut>,
}

/// --------------------
/// Handler
/// --------------------

pub async fn insolation_seasons_handler(
    State(app): State<AppState>,
    Query(q): Query<InsolationSeasonsQuery>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    // Parse inputs
    let uvox = UvoxId::from_hex(&q.uvox)
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let start_time = SimTime(
        q.start_time_ns
            .parse::<i128>()
            .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?,
    );

    let world = q.world;
    let sun = WorldId(0);

    // Load world env
    let world_row = WorldRow::get(&app.supa, world)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    let env = world_row
        .environment
        .as_ref()
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let space: &WorldSpace = &env.space;

    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    // --------------------
    // Sample daily insolation for 1 year
    // --------------------

    let step = SimDuration::days(1);
    let year = SimDuration::years(1);

    let mut samples = Vec::new();
    let mut t = start_time;

    while t.0 < start_time.0 + year.0 {
        let val = daily_insolation(
            &resolver,
            world,
            &uvox,
            sun,
            t,
            space,
            96,
        )
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        samples.push(InsolationSample {
            time: t,
            value: val,
        });

        t = SimTime(t.0 + step.0);
    }

    // --------------------
    // Detect seasons
    // --------------------

    let seasons = detect_insolation_seasons(&samples);

    let response = InsolationSeasonsResponse {
        summer_solstice: seasons.summer_solstice.map(|s| InsolationSeasonOut {
            time_ns: s.time.0,
            insolation: s.insolation,
        }),
        winter_solstice: seasons.winter_solstice.map(|s| InsolationSeasonOut {
            time_ns: s.time.0,
            insolation: s.insolation,
        }),
        vernal_equinox: seasons.vernal_equinox.map(|s| InsolationSeasonOut {
            time_ns: s.time.0,
            insolation: s.insolation,
        }),
        autumnal_equinox: seasons.autumnal_equinox.map(|s| InsolationSeasonOut {
            time_ns: s.time.0,
            insolation: s.insolation,
        }),
    };

    Ok(Json(response))
}

use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::core::tdt::{SimTime, SimDuration};
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_frame::WorldResolver;
use crate::core::world::presets::frames::frame_presets;
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::physics::illumination::solar_illumination;
use crate::supabasic::worlds::WorldRow;
use crate::core::uvoxid::{LatCode, LonCode};

/// One latitude’s seasonal illumination samples
#[derive(Serialize)]
pub struct SeasonSeries {
    pub latitude_deg: f64,
    pub illumination: Vec<f64>, // daily max illumination
}

/// Diagnostic seasons check (daily max solar illumination)
pub async fn seasons_check_handler(
    State(app): State<AppState>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let world = WorldId(1); // Earth
    let sun = WorldId(0);

    let latitudes = [-60.0, -30.0, 0.0, 30.0, 60.0];

    let day_step = SimDuration::days(10);
    let year = SimDuration::years(1);

    let hour_step = SimDuration::hours(1);
    let day_duration = SimDuration::seconds(86164);

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

    let mut results = Vec::new();

    for &lat in &latitudes {
        let lat_code = LatCode((lat * 10_000_000.0) as i64);
        let lon_code = LonCode(0);
        let uvox = UvoxId::earth_surface(lat_code, lon_code);

        let mut samples = Vec::new();

        let mut t_day = SimTime(0);
        while t_day.0 < year.0 {
            let mut max_illum = 0.0;

            let mut t_hour = SimTime(0);
            while t_hour.0 < day_duration.0 {
                let t = SimTime(t_day.0 + t_hour.0);

                let illum = solar_illumination(
                    &resolver,
                    world,
                    &uvox,
                    sun,
                    t,
                    space,
                )
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

                if illum > max_illum {
                    max_illum = illum;
                }

                t_hour = SimTime(t_hour.0 + hour_step.0);
            }

            samples.push(max_illum);
            t_day = SimTime(t_day.0 + day_step.0);
        }

        results.push(SeasonSeries {
            latitude_deg: lat,
            illumination: samples,
        });
    }

    Ok(Json(results))
}

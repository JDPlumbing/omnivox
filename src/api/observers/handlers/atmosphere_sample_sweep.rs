use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::shared::app_state::AppState;
use crate::core::observer::ObserverId;
use crate::core::tdt::SimTime;
use crate::core::world::presets::frames::frame_presets;
use crate::core::world::world_frame::WorldResolver;
use crate::core::env::atmosphere::AtmosphereField;
use crate::supabasic::worlds::WorldRow;

use crate::api::observers::dtos::atmosphere::AtmosphereSweepResponse;
use crate::api::observers::dtos::atmosphere::AtmosphereSweepSample;


use serde::Deserialize;

#[derive(Deserialize)]
pub struct AtmosphereSweepQuery {
    pub time_ns: Option<String>,
    pub start_alt_m: Option<f64>,
    pub end_alt_m: f64,
    pub step_m: f64,
}

pub async fn atmosphere_sweep_handler(
    State(app): State<AppState>,
    Path(observer_id): Path<u64>,
    Query(q): Query<AtmosphereSweepQuery>,
) -> Result<impl IntoResponse, StatusCode> {

    // ----------------------------------
    // Observer
    // ----------------------------------
    let observer = app.observers
        .read().await
        .get(&ObserverId(observer_id))
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    let time = q.time_ns
        .as_deref()
        .unwrap_or("0")
        .parse::<i128>()
        .map(SimTime)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let start_alt = q.start_alt_m.unwrap_or(0.0);
    let end_alt = q.end_alt_m;
    let step = q.step_m;

    if step <= 0.0 || end_alt <= start_alt {
        return Err(StatusCode::BAD_REQUEST);
    }

    // ----------------------------------
    // World + atmosphere
    // ----------------------------------
    let world_row = WorldRow::get(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let env = world_row.environment
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let atmosphere_model = env.atmosphere
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let atmosphere = AtmosphereField::from_model(&env.space, atmosphere_model);

    // ----------------------------------
    // Sweep
    // ----------------------------------
    let mut samples = Vec::new();
    let mut alt = start_alt;

    while alt <= end_alt {
        let r = env.space.surface_radius_m + alt;
        let density = atmosphere.density_at_radius(r);

        samples.push(AtmosphereSweepSample {
            altitude_m: alt,
            density_kg_m3: density,
        });

        alt += step;
    }

    Ok(Json(AtmosphereSweepResponse {
        observer_id,
        time_ns: time.0,
        samples,
    }))
}

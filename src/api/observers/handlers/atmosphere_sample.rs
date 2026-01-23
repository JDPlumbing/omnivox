use serde::Deserialize;

#[derive(Deserialize)]
pub struct AtmosphereSampleQuery {
    pub time_ns: Option<String>,
    pub altitude_m: f64,
}

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

use crate::api::observers::dtos::atmosphere::AtmosphereSampleResponse;

pub async fn atmosphere_sample_handler(
    State(app): State<AppState>,
    Path(observer_id): Path<u64>,
    Query(q): Query<AtmosphereSampleQuery>,
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

    // ----------------------------------
    // World + environment
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

    // ----------------------------------
    // Build atmosphere field
    // ----------------------------------
    let atmosphere = AtmosphereField::from_model(
        &env.space,
        atmosphere_model,
    );

    // ----------------------------------
    // Sample at altitude
    // ----------------------------------
    let r = env.space.surface_radius_m + q.altitude_m;
    let density = atmosphere.density_at_radius(r);

    Ok(Json(AtmosphereSampleResponse {
        observer_id,
        time_ns: time.0,
        altitude_m: q.altitude_m,
        density_kg_m3: density,
    }))
}

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::shared::app_state::AppState;
use crate::supabasic::worlds::WorldRow;
use crate::api::observers::dtos::SurfaceEnergyResponse;

use crate::core::observer::ObserverId;
use crate::core::tdt::SimTime;
use crate::core::world::presets::frames::frame_presets;
use crate::core::world::world_frame::WorldResolver;
use crate::core::physics::surface_energy::surface_solar_irradiance;

#[derive(Deserialize)]
pub struct SurfaceEnergyQuery {
    pub time_ns: Option<String>,
}

pub async fn surface_energy_handler(
    State(app): State<AppState>,
    Path(observer_id): Path<u64>,
    Query(q): Query<SurfaceEnergyQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    // ----------------------------
    // Observer
    // ----------------------------
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

    // ----------------------------
    // World + environment
    // ----------------------------
    let world_row = WorldRow::get(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let env = world_row.environment
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    // ----------------------------
    // Resolver
    // ----------------------------
    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    // ----------------------------
    // Surface energy
    // ----------------------------
    let irradiance = surface_solar_irradiance(
        &resolver,
        observer.world,
        &observer.uvox,
        time,
        &env.space,
    ).map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(SurfaceEnergyResponse {
        observer_id,
        time_ns: time.0,
        direct_w_m2: irradiance.direct_w_m2,
        diffuse_w_m2: irradiance.diffuse_w_m2,
        total_w_m2: irradiance.total_w_m2,
    }))
}

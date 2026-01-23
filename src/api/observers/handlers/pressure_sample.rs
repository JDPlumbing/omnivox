use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::shared::app_state::AppState;
use crate::core::observer::ObserverId;
use crate::core::tdt::{SimTime, SimDuration};
use crate::core::world::WorldEnvironment;
use crate::supabasic::worlds::WorldRow;

use crate::api::observers::dtos::pressure::PressureSampleDto;

#[derive(Deserialize)]
pub struct PressureSampleQuery {
    pub time_ns: Option<String>,
}

pub async fn pressure_sample_handler(
    State(app): State<AppState>,
    Path(observer_id): Path<u64>,
    Query(q): Query<PressureSampleQuery>,
) -> Result<impl IntoResponse, StatusCode> {

    // Observer
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

    // World
    let world_row = WorldRow::get(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let env_desc = world_row.environment
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let world_env = WorldEnvironment::from_descriptor(env_desc);

    // Sample
    let duration = SimDuration(time.0);

    let sample = world_env.sample(&observer.uvox, duration);


    Ok(Json(PressureSampleDto {
        pressure_pa: sample.pressure,
    }))
}

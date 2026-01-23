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
use crate::core::UvoxId;

use crate::api::observers::dtos::pressure::PressureSweepSample;

#[derive(Deserialize)]
pub struct PressureSweepQuery {
    pub time_ns: Option<String>,
    pub start_alt_m: Option<f64>,
    pub end_alt_m: f64,
    pub step_m: f64,
}

pub async fn pressure_sweep_handler(
    State(app): State<AppState>,
    Path(observer_id): Path<u64>,
    Query(q): Query<PressureSweepQuery>,
) -> Result<impl IntoResponse, StatusCode> {

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

    let world_row = WorldRow::get(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let env_desc = world_row.environment
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let world_env = WorldEnvironment::from_descriptor(env_desc);

    let mut samples = Vec::new();
    let mut alt = start_alt;

let duration = SimDuration(time.0);

let base_vec = observer.uvox.to_vec3(); // ← you already have this or equivalent
let base_r = observer.uvox.radius_m();
let dir = {
    let mag = base_r as f32;
    [
        base_vec[0] / mag,
        base_vec[1] / mag,
        base_vec[2] / mag,
    ]
};

while alt <= end_alt {
    let r = env_desc.space.surface_radius_m + alt;

    let probe_vec = [
        dir[0] * r as f32,
        dir[1] * r as f32,
        dir[2] * r as f32,
    ];

    let probe_uvox = UvoxId::from_vec3(probe_vec);

    let sample = world_env.sample(&probe_uvox, duration);

    samples.push(PressureSweepSample {
        altitude_m: alt,
        pressure_pa: sample.pressure,
    });

    alt += step;
}



    Ok(Json(samples))
}

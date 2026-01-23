use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;

use crate::shared::app_state::AppState;
use crate::core::observer::ObserverId;
use crate::core::tdt::{SimTime, SimDuration};
use crate::core::world::WorldEnvironment;
use crate::core::env::chemistry::{AtmosphereChemistry, Species};
use crate::supabasic::worlds::WorldRow;

use crate::api::observers::dtos::chemistry::AtmosphereChemistryResponse;

#[derive(Deserialize)]
pub struct ChemistryQuery {
    pub time_ns: Option<String>,
}

pub async fn chemistry_atmosphere_handler(
    State(app): State<AppState>,
    Path(observer_id): Path<u64>,
    Query(q): Query<ChemistryQuery>,
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

    let duration = SimDuration(time.0);

    // ----------------------------------
    // World + environment
    // ----------------------------------
    let world_row = WorldRow::get(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let env_desc = world_row.environment
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let world_env = WorldEnvironment::from_descriptor(env_desc);
    let env_sample = world_env.sample(&observer.uvox, duration);

    // ----------------------------------
    // Chemistry
    // ----------------------------------
    let chemistry = AtmosphereChemistry::earth_like();
    let chem_sample = chemistry.sample(&env_sample);

    // ----------------------------------
    // Serialize species names
    // ----------------------------------
    let mut partial_pressure_pa = HashMap::new();
    let mut mass_density_kg_m3 = HashMap::new();

    for (species, value) in chem_sample.partial_pressure_pa {
        partial_pressure_pa.insert(format!("{:?}", species), value);
    }

    for (species, value) in chem_sample.mass_density_kg_m3 {
        mass_density_kg_m3.insert(format!("{:?}", species), value);
    }

    Ok(Json(AtmosphereChemistryResponse {
        observer_id,
        time_ns: time.0,
        partial_pressure_pa,
        mass_density_kg_m3,
    }))
}

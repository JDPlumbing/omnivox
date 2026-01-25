use serde_json::json;
use axum::extract::{State, Path, Json };
use axum::response::IntoResponse;
use reqwest::StatusCode;
use crate::core::ObserverId;
use crate::shared::AppState;
use crate::supabasic::WorldRow;
use crate::core::world::WorldEnvironment;
use crate::core::SimDuration;


pub async fn land_height_handler(
    State(app): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, StatusCode> {
    let observer = app.observers
        .read().await
        .get(&ObserverId(id))
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    let world_row = WorldRow::get(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let env_desc = world_row.environment
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let world_env = WorldEnvironment::from_descriptor(env_desc);

    let sample = world_env.sample(&observer.uvox, SimDuration::ZERO);

    Ok(Json(json!({
        "observer_id": id,
        "land_height_m": sample.land_height_m
    })))
}

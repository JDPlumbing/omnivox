use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::core::uvoxid::UvoxId;
use crate::core::tdt::sim_duration::SimDuration;
use crate::engine::world::loader::load_world;
use crate::api::worlds::dto::environment::EnvSampleDto;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct EnvSampleQuery {
    pub uvox: String,
    pub time_ns: Option<i128>,
}

pub async fn sample_environment_handler(
    State(app): State<AppState>,
    Path(world_id): Path<WorldId>,
    Query(q): Query<EnvSampleQuery>,
) -> impl IntoResponse {

    // 1️⃣ Parse UvoxId
    let uvox = match UvoxId::from_str(&q.uvox) {
        Ok(id) => id,
        Err(_) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "invalid uvox id" })),
            )
                .into_response();
        }
    };

    let time = SimDuration::from_ns(q.time_ns.unwrap_or(0));

    // 2️⃣ Load world (this hydrates WorldEnvironment)
    let world = match load_world(&app.supa, world_id).await {
        Ok(w) => w,
        Err(e) => {
            return (
                axum::http::StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": format!("{e}") })),
            )
                .into_response();
        }
    };

    // 3️⃣ Sample environment
    let snapshot = world
        .environment
        .sample(&uvox, time);

    // 4️⃣ Return DTO
    Json(EnvSampleDto {
        medium: snapshot.medium,
        density: snapshot.density,
        gravity_radial: snapshot.gravity_radial,
        pressure: snapshot.pressure,
        temperature: snapshot.temperature,
    })
    .into_response()
}

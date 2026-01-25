use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::str::FromStr;
use std::sync::Arc;

use crate::shared::app_state::AppState;
use crate::core::id::WorldId;
use crate::core::uvoxid::UvoxId;
use crate::core::tdt::sim_duration::SimDuration;
use crate::api::worlds::dtos::environment::EnvSampleDto;
use crate::engine::world::state::WorldState;

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
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "invalid uvox id"
                })),
            )
                .into_response();
        }
    };

    let time = SimDuration::from_ns(q.time_ns.unwrap_or(0));

    // 2️⃣ Try runtime cache first
    let cached: Option<Arc<WorldState>> = {
        let worlds = app.worlds.read().await;
        worlds.get(&world_id).cloned()
    };

    // 3️⃣ Load from source if missing
    let world: Arc<WorldState> = match cached {
        Some(w) => w,
        None => {
            match app.world_source.load_world(world_id).await {
                Ok(loaded) => {
                    let arc = Arc::new(loaded);
                    app.worlds.write().await.insert(world_id, arc.clone());
                    arc
                }
                Err(e) => {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(serde_json::json!({
                            "error": e.to_string()
                        })),
                    )
                        .into_response();
                }
            }
        }
    };

    // 4️⃣ Sample environment
    let snapshot = world.environment.sample(&uvox, time);

    // 5️⃣ Return DTO
    Json(EnvSampleDto {
        medium: snapshot.medium,
        density: snapshot.density,
        gravity_radial: snapshot.gravity_radial,
        pressure: snapshot.pressure,
        temperature: snapshot.temperature,
    })
    .into_response()
}

use axum::{
    extract::State,
    Json,
};

use crate::shared::app_state::AppState;
use crate::engine::entities::entity_engine::EntityEngine;
use crate::core::components::time::Time;
use crate::core::SimTime;

use crate::api::entities::dtos::create_time::CreateTimeEntity;

pub async fn create_time_entity(
    State(app): State<AppState>,
    Json(payload): Json<CreateTimeEntity>,
) -> Json<serde_json::Value> {

    // 🔐 write-lock ECS store
    let mut store = app.entity_store.write().await;

    // ⚙️ entity engine owns creation
    let mut engine = EntityEngine::new(&mut store);

    let entity_id = engine.create_time_marker(Time {
        sim_time: SimTime(payload.sim_time),
    });

    Json(serde_json::json!({
        "entity_id": entity_id.to_string()
    }))
}

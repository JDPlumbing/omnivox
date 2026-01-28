use axum::{Json, extract::State};
use serde::Deserialize;

use crate::shared::app_state::AppState;

#[derive(Deserialize)]
pub struct DeltaRequest {
    pub start_ns: String,
    pub end_ns: String,
}

pub async fn delta_handler(
    State(app): State<AppState>,
    Json(req): Json<DeltaRequest>,
) -> Json<serde_json::Value> {
    let start_ns: i128 = req.start_ns.parse().unwrap_or(0);
    let end_ns: i128 = req.end_ns.parse().unwrap_or(0);

    let result = app.time_engine.delta_between(start_ns, end_ns);

    Json(serde_json::json!({
        "delta_ns": result.delta_ns.to_string(),
        "human": result.human,
    }))
}

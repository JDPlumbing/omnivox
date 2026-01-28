use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::shared::app_state::AppState;

#[derive(Deserialize)]
pub struct DurationQuery {
    pub ns: String,
}

pub async fn duration_human_handler(
    State(app): State<AppState>,
    Query(q): Query<DurationQuery>,
) -> Json<serde_json::Value> {
    let ns: i128 = q.ns.parse().unwrap_or(0);

    let result = app.time_engine.human_duration(ns);

    Json(serde_json::json!({
        "human": result.human,
        "ns": result.ns.to_string(),
    }))
}

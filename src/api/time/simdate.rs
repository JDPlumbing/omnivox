use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::shared::app_state::AppState;

#[derive(Deserialize)]
pub struct SimdateQuery {
    pub ns: String,
}

pub async fn simdate_from_ns_handler(
    State(app): State<AppState>,
    Query(q): Query<SimdateQuery>,
) -> Json<serde_json::Value> {
    let ns: i128 = q.ns.parse().unwrap_or(0);

    let result = app.time_engine.simdate_from_ns(ns);

    Json(serde_json::json!({
        "simdate": result.simdate,
        "ns": result.ns.to_string(),
    }))
}

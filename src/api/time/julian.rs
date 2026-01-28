use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::shared::app_state::AppState;

#[derive(Deserialize)]
pub struct JulianQuery {
    pub ns: String,
}

pub async fn julian_from_ns_handler(
    State(app): State<AppState>,
    Query(q): Query<JulianQuery>,
) -> Json<serde_json::Value> {
    let ns: i128 = q.ns.parse().unwrap_or(0);

    let result = app.time_engine.julian_from_ns(ns);

    Json(serde_json::json!({
        "julian_date": result.julian_date,
        "ns": result.ns.to_string(),
    }))
}

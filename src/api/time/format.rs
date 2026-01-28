use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::shared::app_state::AppState;
use crate::core::tdt::sim_display::TimeFormat;

#[derive(Deserialize)]
pub struct FormatQuery {
    pub ns: String,
    pub fmt: TimeFormat,
}

pub async fn format_simtime_handler(
    State(app): State<AppState>,
    Query(q): Query<FormatQuery>,
) -> Json<serde_json::Value> {
    let ns: i128 = q.ns.parse().unwrap_or(0);

    let result = app.time_engine.format_simtime(ns, q.fmt);

    Json(serde_json::json!({
        "formatted": result.formatted,
        "format": result.format,
        "ns": result.ns.to_string(),
    }))
}

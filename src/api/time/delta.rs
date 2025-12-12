use axum::{Json};
use serde::Deserialize;
use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_duration::SimDuration;

#[derive(Deserialize)]
pub struct DeltaRequest {
    pub start_ns: String,
    pub end_ns: String,
}

pub async fn delta_handler(Json(req): Json<DeltaRequest>) -> Json<serde_json::Value> {
    let start = SimTime::from_ns(req.start_ns.parse().unwrap_or(0));
    let end = SimTime::from_ns(req.end_ns.parse().unwrap_or(0));

    let delta: SimDuration = end - start;

    Json(serde_json::json!({
        "delta_ns": delta.as_ns().to_string(),
        "human": delta.to_string_human()
    }))
}

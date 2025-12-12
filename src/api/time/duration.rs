use axum::{Json, extract::Query};
use serde::Deserialize;
use crate::core::tdt::sim_duration::SimDuration;

#[derive(Deserialize)]
pub struct DurationQuery {
    pub ns: String,   // <-- STRING
}

pub async fn duration_human_handler(Query(q): Query<DurationQuery>) -> Json<serde_json::Value> {
    let ns: i128 = q.ns.parse().unwrap_or(0);
    let dur = SimDuration::from_ns(ns);

    Json(serde_json::json!({
        "human": dur.to_string_human(),
        "ns": ns.to_string()
    }))
}

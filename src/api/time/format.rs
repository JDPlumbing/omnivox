use axum::{Json, extract::Query};
use serde::Deserialize;
use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_display::{format_simtime, TimeFormat};

#[derive(Deserialize)]
pub struct FormatQuery {
    pub ns: String,     // <-- STRING incoming
    pub fmt: TimeFormat,
}

pub async fn format_simtime_handler(Query(q): Query<FormatQuery>) -> Json<serde_json::Value> {
    // Parse safely
    let ns: i128 = q.ns.parse().unwrap_or(0);
    let t = SimTime::from_ns(ns);

    Json(serde_json::json!({
        "formatted": format_simtime(t, q.fmt),
        "format": q.fmt,
        "ns": ns.to_string()      // <-- return string
    }))
}

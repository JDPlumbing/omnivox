use axum::{Json, extract::Query};
use serde::Deserialize;
use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_display::format_simdate;

#[derive(Deserialize)]
pub struct SimdateQuery {
    pub ns: String,
}

pub async fn simdate_from_ns_handler(Query(q): Query<SimdateQuery>) -> Json<serde_json::Value> {
    let ns: i128 = q.ns.parse().unwrap_or(0);
    let t = SimTime::from_ns(ns);

    Json(serde_json::json!({
        "simdate": format_simdate(t),
        "ns": ns.to_string()
    }))
}

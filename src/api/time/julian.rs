use axum::{Json, extract::Query};
use serde::Deserialize;
use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_julian::simtime_to_julian;

#[derive(Deserialize)]
pub struct JulianQuery {
    pub ns: String,
}

pub async fn julian_from_ns_handler(Query(q): Query<JulianQuery>) -> Json<serde_json::Value> {
    let ns: i128 = q.ns.parse().unwrap_or(0);
    let t = SimTime::from_ns(ns);
    let jd = simtime_to_julian(t);

    Json(serde_json::json!({
        "julian_date": jd,
        "ns": ns.to_string()
    }))
}

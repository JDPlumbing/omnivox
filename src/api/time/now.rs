use axum::response::IntoResponse;
use axum::Json;
use crate::core::tdt::sim_time::SimTime;


pub async fn simtime_now() -> impl IntoResponse {
    let now = SimTime::now();
    Json(serde_json::json!({
        "simtime_ns": now.as_ns(),
        "datetime": now.to_datetime().to_rfc3339(),
    }))
}

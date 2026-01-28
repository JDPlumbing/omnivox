use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};

use crate::shared::app_state::AppState;

pub async fn simtime_now(
    State(app): State<AppState>,
) -> impl IntoResponse {
    let now = app.time_engine.now();

    Json(serde_json::json!({
        "simtime_ns": now.as_ns().to_string(),
        "datetime": now.to_datetime().to_rfc3339(),
    }))
}

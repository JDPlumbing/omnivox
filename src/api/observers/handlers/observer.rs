use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::shared::app_state::AppState;
use crate::api::observers::dtos::*;
use crate::core::observer::{Observer, ObserverId};
use crate::core::uvoxid::UvoxId;
use crate::core::tdt::{SimTime, SimDuration};

static OBSERVER_SEQ: AtomicU64 = AtomicU64::new(1);

pub async fn create_observer(
    State(app): State<AppState>,
    Json(req): Json<CreateObserverRequest>,
) -> impl IntoResponse {
    let uvox = match UvoxId::from_hex(&req.uvox) {
        Some(u) => u,
        None => return StatusCode::BAD_REQUEST.into_response(),
    };

    let id = ObserverId(OBSERVER_SEQ.fetch_add(1, Ordering::Relaxed));

    let observer = Observer {
        id,
        world: req.world,
        uvox,
        created_at: SimTime(0),
    };

    app.observers.write().await.insert(id, observer.clone());

    Json(ObserverResponse {
        id: observer.id.0,
        world: observer.world,
        uvox: observer.uvox.to_hex(),
    })
    .into_response()
}

pub async fn get_observer(
    State(app): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, StatusCode> {
    let observer = app.observers
        .read().await
        .get(&ObserverId(id))
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ObserverResponse {
        id: observer.id.0,
        world: observer.world,
        uvox: observer.uvox.to_hex(),
    }))
}

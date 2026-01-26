use axum::{
    //debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::{json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::supabasic::addresses::AddressRow;
//use crate::sim::address::Address;
use crate::shared::app_state::AppState;



/// Data model for creating a new address
#[derive(Debug, Serialize, Deserialize)]
pub struct NewAddress {
    pub street_address: String,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

// ========================================================
// GET /address
// ========================================================
pub async fn list_addresses(
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.address_source.list().await {
        Ok(rows) => Json(rows).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}


// ========================================================
// GET /address/{id}
// ========================================================
pub async fn get_address(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.address_source.get(id).await {
        Ok(Some(addr)) => Json(addr).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}


// ========================================================
// POST /address
// ========================================================
pub async fn create_address(
    State(state): State<AppState>,
    Json(input): Json<NewAddress>,
) -> impl IntoResponse {
    let record = AddressRow {
        id: None,
        street_address: Some(input.street_address.trim().to_lowercase()),
        city: input.city.map(|c| c.trim().to_lowercase()),
        state: input.state.map(|s| s.trim().to_lowercase()),
        postal_code: input.postal_code.map(|p| p.trim().to_lowercase()),
        country: Some(input.country.unwrap_or("us".into()).trim().to_lowercase()),
    };

    match state.address_source.create(record).await {
        Ok(inserted) => Json(inserted).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}


// ========================================================
// PUT /address/{id}
// ========================================================
pub async fn update_address(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(update): Json<AddressUpdate>,
) -> impl IntoResponse {
    let payload = serde_json::to_value(&update).unwrap();

    match state.address_source.update(id, payload).await {
        Ok(updated) => Json(updated).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}


// ========================================================
// PATCH /address/{id}
// ========================================================
pub async fn patch_address(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    if payload.as_object().map(|m| m.is_empty()).unwrap_or(true) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Empty patch payload" })),
        )
            .into_response();
    }

    match state.address_source.update(id, payload).await {
        Ok(updated) => Json(json!({ "patched": updated })).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}


// ========================================================
// DELETE /address/{id}
// ========================================================
pub async fn delete_address(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.address_source.delete(id).await {
        Ok(_) => Json(json!({ "status": "deleted", "id": id })).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}


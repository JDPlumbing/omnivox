use axum::{
    //debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::supabasic::addresses::AddressRow;
//use crate::sim::address::Address;
use crate::shared::app_state::AppState;
use crate::api::geocode:: {resolve_address};


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
pub async fn list_addresses(State(app): State<AppState>) -> impl IntoResponse {
    let result = app
        .supa
        .from("addresses")
        .select("id, street_address, city, state, postal_code, country")
        .execute_typed::<AddressRow>()
        .await;

    match result {
        Ok(rows) => Json(rows).into_response(),
        Err(e) => {
            eprintln!("Error listing addresses: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "error listing addresses" })),
            )
                .into_response()
        }
    }
}

// ========================================================
// GET /address/{id}
// ========================================================
pub async fn get_address(State(app): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let result = app
        .supa
        .from("addresses")
        .select("id, street_address, city, state, postal_code, country")
        .eq("id", &id.to_string())
        .single_typed::<AddressRow>()
        .await;

    match result {
        Ok(row) => Json(row).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": format!("not found: {e:?}") })),
        )
            .into_response(),
    }
}

// ========================================================
// POST /address
// ========================================================
pub async fn create_address(
    State(app): State<AppState>,
    Json(addr): Json<NewAddress>,
) -> impl IntoResponse {
    // Normalize address
    let street = addr.street_address.trim().to_lowercase();
    let city = addr.city.clone().unwrap_or_default().trim().to_lowercase();
    let state = addr.state.clone().unwrap_or_default().trim().to_lowercase();
    let postal = addr.postal_code.clone().unwrap_or_default().trim().to_lowercase();
    let country = addr.country.clone().unwrap_or("US".to_string()).trim().to_lowercase();

    // Step 1️⃣: Check if it already exists
    let existing_result = app
        .supa
        .from("addresses")
        .select("id, street_address, city, state, postal_code, country")
        .eq("street_address", &street)
        .eq("city", &city)
        .eq("state", &state)
        .eq("postal_code", &postal)
        .eq("country", &country)
        .execute_typed::<AddressRow>()
        .await;

    if let Ok(rows) = &existing_result {
        if !rows.is_empty() {
            let found = &rows[0];
            eprintln!("♻️ Found existing address {:?}", found);
            return Json(json!({
                "status": "ok",
                "existing": true,
                "inserted": found
            }))
            .into_response();
        }
    }

    // Step 2️⃣: Create if not found
    let record = AddressRow {
        id: None,
        street_address: Some(street.clone()),
        city: Some(city),
        state: Some(state),
        postal_code: Some(postal),
        country: Some(country),
    };

    let result = AddressRow::create(&app.supa, &record).await;

    match result {
        Ok(inserted) => Json(json!({ "status": "ok", "existing": false, "inserted": inserted })).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Insert failed: {e:?}") })),
        )
            .into_response(),
    }
}



// ========================================================
// PUT /address/{id}
// ========================================================
pub async fn update_address(
    State(app): State<AppState>,
    Path(id): Path<Uuid>,
    Json(update): Json<AddressUpdate>,
) -> impl IntoResponse {
    let payload = serde_json::to_value(&update).unwrap();

    let result = app
        .supa
        .from("addresses")
        .eq("id", &id.to_string())
        .update(payload)
        .select("*")
        .execute_typed::<AddressRow>()
        .await;

    match result {
        Ok(mut updated) => {
            if updated.is_empty() {
                return (StatusCode::NOT_FOUND, "No row updated").into_response();
            }
            Json(json!({ "updated": updated.remove(0) })).into_response()
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Update failed: {e:?}") })),
        )
            .into_response(),
    }
}

// ========================================================
// PATCH /address/{id}
// ========================================================
pub async fn patch_address(
    State(app): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    if payload.as_object().map(|m| m.is_empty()).unwrap_or(true) {
        return (StatusCode::BAD_REQUEST, "Empty patch payload").into_response();
    }

    match app
        .supa
        .from("addresses")
        .eq("id", &id.to_string())
        .update(json!(payload))
        .select("*")
        .execute_typed::<AddressRow>()
        .await
    {
        Ok(rows) => Json(json!({ "patched": rows })).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Patch failed: {e:?}") })),
        )
            .into_response(),
    }
}

// ========================================================
// DELETE /address/{id}
// ========================================================
pub async fn delete_address(State(app): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let result = app
        .supa
        .from("addresses")
        .eq("id", &id.to_string())
        .delete()
        .execute()
        .await;

    match result {
        Ok(_) => Json(json!({ "status": "deleted", "id": id })).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Delete failed: {e:?}") })),
        )
            .into_response(),
    }
}

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
// POST /address/{id}/resolve (Find or create geolocation + uvoxid)
// ========================================================
pub async fn resolve_address(State(app): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    // Step 1: Fetch the address
    let addr: AddressRow = match app
        .supa
        .from("addresses")
        .select("id, street_address, city, state, postal_code, country")
        .eq("id", &id.to_string())
        .single_typed::<AddressRow>()
        .await
    {
        Ok(row) => row,
        Err(e) => {
            eprintln!("Address not found: {:?}", e);
            return (StatusCode::NOT_FOUND, "Address not found").into_response();
        }
    };

    // Step 2: Check if we already have a geolocation for this address
    if let Ok(existing) = app
        .supa
        .from("geolocations")
        .select("id, lat, lon, elevation_m")
        .eq("address_id", &id.to_string())
        .single()
        .await
    {
        eprintln!("♻️ Using existing geolocation for address {}", id);
        return Json(json!({
            "status": "ok",
            "reused": true,
            "geolocation": existing,
        }))
        .into_response();
    }

    // Step 3: Geocode using OpenCage
    let query = format!(
        "{}, {}, {}, {}",
        addr.street_address.clone().unwrap_or_default(),
        addr.city.clone().unwrap_or_default(),
        addr.state.clone().unwrap_or_default(),
        addr.country.clone().unwrap_or_default()
    );

    let api_key = std::env::var("OPENCAGE_API_KEY").unwrap_or_else(|_| "MISSING_KEY".to_string());
    let url = format!(
        "https://api.opencagedata.com/geocode/v1/json?q={}&key={}",
        urlencoding::encode(&query),
        api_key
    );

    let client = reqwest::Client::new();
    let resp = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("OpenCage error: {:?}", e);
            return (StatusCode::BAD_REQUEST, "Failed to contact geocoding service").into_response();
        }
    };

    let data: Value = match resp.json().await {
        Ok(d) => d,
        Err(e) => {
            eprintln!("JSON parse error: {:?}", e);
            return (StatusCode::BAD_REQUEST, "Bad geocoding response").into_response();
        }
    };

    let Some(result) = data["results"].get(0) else {
        return (StatusCode::BAD_REQUEST, "No geocode results").into_response();
    };

    let lat = result["geometry"]["lat"].as_f64().unwrap_or(0.0);
    let lon = result["geometry"]["lng"].as_f64().unwrap_or(0.0);
    let elevation_m = 0.0;

    // Step 4: Insert geolocation
    let geo_result = app
        .supa
        .from("geolocations")
        .insert(json!({
            "lat": lat,
            "lon": lon,
            "elevation_m": elevation_m,
            "address_id": id
        }))
        .select("id, lat, lon, elevation_m")
        .execute()
        .await;

    let geo_value: Value = match geo_result {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error inserting geolocation: {:?}", e);
            return (StatusCode::BAD_REQUEST, format!("Insert failed: {e:?}")).into_response();
        }
    };

    let geolocation_id = geo_value[0]["id"].as_str().unwrap_or_default().to_string();

// Step 5: Compute uvoxid fields
const EARTH_RADIUS_M: f64 = 6_371_000.0;
let lat_code: i64 = (lat * 1e9) as i64;
let lon_code: i64 = (lon * 1e9) as i64;
let r_um: i64 = ((EARTH_RADIUS_M + elevation_m) * 1e6) as i64;
let frame_id: i64 = 0;

// Step 6: Check if a uvoxid already exists for these coordinates
let existing_uvox = app
    .supa
    .from("uvoxid")
    .select("frame_id, r_um, lat_code, lon_code, geolocation_id")
    .eq("frame_id", &frame_id.to_string())
    .eq("r_um", &r_um.to_string())
    .eq("lat_code", &lat_code.to_string())
    .eq("lon_code", &lon_code.to_string())
    .execute()
    .await;

if let Ok(val) = &existing_uvox {
    if let Some(arr) = val.as_array() {
        if !arr.is_empty() {
            eprintln!("✅ Existing uvoxid found, skipping insert.");
            return Json(json!({
                "status": "ok",
                "reused": true,
                "geolocation": geo_value,
                "uvoxid": arr[0],
                "lat": lat,
                "lon": lon,
                "r_um": r_um
            }))
            .into_response();
        }
    }
}

// Step 7: Otherwise insert new uvoxid
let uvox_result = app
    .supa
    .from("uvoxid")
    .insert(json!({
        "frame_id": frame_id,
        "r_um": r_um,
        "lat_code": lat_code,
        "lon_code": lon_code,
        "geolocation_id": geolocation_id
    }))
    .select("frame_id, r_um, lat_code, lon_code, geolocation_id")
    .execute()
    .await;

match uvox_result {
    Ok(v) => Json(json!({
        "status": "ok",
        "reused": false,
        "geolocation": geo_value,
        "uvoxid": v,
        "lat": lat,
        "lon": lon,
        "r_um": r_um
    }))
    .into_response(),
    Err(e) => (
        StatusCode::BAD_REQUEST,
        Json(json!({ "error": format!("Uvox insert failed: {e:?}") })),
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

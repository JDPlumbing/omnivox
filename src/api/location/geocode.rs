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
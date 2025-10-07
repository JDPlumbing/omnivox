use axum::{extract::Path, response::IntoResponse, Json};
use axum::http::StatusCode;
use serde_json::{Value, json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::supabasic::Supabase;
use crate::supabasic::addresses::AddressRow;
use crate::supabasic::SupabasicError;
use crate::sim::address::Address;

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

/// GET /address
pub async fn list_addresses() -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    let result = supa
        .from("addresses")
        .select("id, street_address, city, state, postal_code, country")
        .execute_typed::<AddressRow>()
        .await;
    println!("🔎 list_addresses DB call result: {:?}", result);

    match result {
        Ok(rows) => Json(rows).into_response(),
        Err(e) => {
            eprintln!("Error listing addresses: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error listing addresses").into_response()
        }
    }
}

/// GET /address/{id}
pub async fn get_address(Path(id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    let result = supa
        .from("addresses")
        .select("id, street_address, city, state, postal_code, country")
        .eq("id", &id.to_string())
        .single_typed::<AddressRow>()
        .await;
    println!("🔎 get_address DB call for id {}: {:?}", id, result);

    match result {
        Ok(row) => Json(row).into_response(),
        Err(e) => {
            eprintln!("Error fetching address {}: {:?}", id, e);
            (StatusCode::NOT_FOUND, "address not found").into_response()
        }
    }
}

/// POST /address
/// POST /api/address
pub async fn create_address(Json(addr): Json<NewAddress>) -> impl IntoResponse {
    println!("📬 create_address hit with {:?}", addr);
    let supa = Supabase::new_from_env().unwrap();

    let record = AddressRow {
        id: None,
        street_address: Some(addr.street_address),

        city: addr.city,
        state: addr.state,
        postal_code: addr.postal_code,
        country: addr.country,
    };

    let result = AddressRow::create(&supa, &record).await;
    println!("📝 create_address DB insert result: {:?}", result);

    match result {
        Ok(inserted) => Json(json!({ "status": "ok", "inserted": inserted })).into_response(),
        Err(e) => {
            eprintln!("Error inserting address: {:?}", e);
            (StatusCode::BAD_REQUEST, format!("Insert failed: {e:?}")).into_response()
        }
    }
}

/// POST /api/address/{id}/resolve
pub async fn resolve_address(Path(id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    // 1️⃣ Fetch address row
    let addr: AddressRow = match supa
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

    // 2️⃣ Build OpenCage query
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
    let resp = client.get(&url).send().await.unwrap();
    let data: serde_json::Value = resp.json().await.unwrap();
    let Some(result) = data["results"].get(0) else {
        return (StatusCode::BAD_REQUEST, "No geocode results").into_response();
    };

    let lat = result["geometry"]["lat"].as_f64().unwrap_or(0.0);
    let lon = result["geometry"]["lng"].as_f64().unwrap_or(0.0);
    let elevation_m = 0.0;

    // 3️⃣ Insert into geolocations (working version)
    let geo_result = supa
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

    if let Err(e) = &geo_result {
        eprintln!("Error inserting geolocation: {:?}", e);
        return (StatusCode::BAD_REQUEST, format!("Insert failed: {e:?}")).into_response();
    }

    let geo_value: serde_json::Value = geo_result.unwrap();
    let geolocation_id = geo_value[0]["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    // 4️⃣ Compute uvoxid fields
    const EARTH_RADIUS_M: f64 = 6_371_000.0;
    let lat_code: i64 = (lat * 1e9) as i64;
    let lon_code: i64 = (lon * 1e9) as i64;
    let r_um: i64 = ((EARTH_RADIUS_M + elevation_m) * 1e6) as i64;
    let frame_id: i64 = 0;

    println!(
        "🧭 Calculated UVXOID: frame={}, r_um={}, lat_code={}, lon_code={}, geolocation_id={}",
        frame_id, r_um, lat_code, lon_code, geolocation_id
    );

    // 5️⃣ Insert into uvoxid (same JSON shape as before)
    let uvox_result = supa
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

    println!("📡 resolve_address uvoxid insert result: {:?}", uvox_result);

    match uvox_result {
        Ok(v) => Json(json!({
            "status": "ok",
            "geolocation": geo_value,
            "uvoxid": v,
            "lat": lat,
            "lon": lon,
            "r_um": r_um
        }))
        .into_response(),
        Err(e) => {
            eprintln!("Error inserting uvoxid: {:?}", e);
            (StatusCode::BAD_REQUEST, format!("Uvox insert failed: {e:?}")).into_response()
        }
    }
}

/// PUT /address/{id}
pub async fn update_address(
    Path(id): Path<Uuid>,
    Json(update): Json<AddressUpdate>,
) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();
    let payload = serde_json::to_value(&update).unwrap();

    let result = supa
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
        Err(e) => {
            eprintln!("Error updating address {}: {:?}", id, e);
            (StatusCode::BAD_REQUEST, format!("Update failed: {e:?}")).into_response()
        }
    }
}


/// PATCH /address/{id}
pub async fn patch_address(
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let supa = match Supabase::new_from_env() {
        Ok(client) => client,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Supabase init error: {e}")).into_response(),
    };

    // Defensive check: empty payload means nothing to patch
    if payload.as_object().map(|m| m.is_empty()).unwrap_or(true) {
        return (StatusCode::BAD_REQUEST, "Empty patch payload".to_string()).into_response();
    }

    // Ensure we’re wrapping the payload correctly for PostgREST
    let filtered = json!(payload);

    eprintln!("📦 PATCH filtered payload: {}", filtered);

    // Chain the entire builder sequence in one expression
    match supa
        .from("addresses")
        .eq("id", &id.to_string()) // ✅ filter first
        .update(filtered)
        .select("*")
        .execute_typed::<AddressRow>()
        .await

    {
        Ok(rows) => Json(json!({ "patched": rows })).into_response(),
        Err(e) => {
            eprintln!("Error patching address {id}: {e:?}");
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Patch failed: {e:?}")).into_response()
        }
    }
}

/// DELETE /address/{id}
pub async fn delete_address(Path(id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    let result = supa
        .from("addresses")
        .eq("id", &id.to_string())
        .delete()
        .execute()
        .await;
    println!("🗑️ delete_address DB delete result for id {}: {:?}", id, result);

    match result {
        Ok(_) => Json(json!({ "status": "deleted", "id": id })).into_response(),
        Err(e) => {
            eprintln!("Error deleting address {}: {:?}", id, e);
            (StatusCode::BAD_REQUEST, format!("Delete failed: {e:?}")).into_response()
        }
    }
}

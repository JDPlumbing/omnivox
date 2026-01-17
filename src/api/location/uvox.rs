use axum::{
    extract::{State, Json},
    //http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::shared::app_state::AppState;
use crate::core::uvoxid::{UvoxId, LatCode, LonCode, RUm, EARTH_RADIUS_UM, ANG_SCALE};

//
// ------------------------
// 1. POST /uvox/from_coords
// ------------------------
// Convert Earth lat/lon/elevation → UVoxId
//

#[derive(Debug, Deserialize)]
pub struct CoordsToUvoxPayload {
    pub lat: f64,        // degrees
    pub lon: f64,        // degrees
    pub elevation_m: f64 // meters above sea level
}

pub async fn coords_to_uvox(
    State(_): State<AppState>,
    Json(body): Json<CoordsToUvoxPayload>,
) -> impl IntoResponse {

    // Convert elevation into µm radius:
    // r_um = (earth_radius + elevation_m) * 1e6
    let r_um_i64 = ((body.elevation_m + (EARTH_RADIUS_UM as f64 / 1_000_000.0)) * 1_000_000.0) as i64;

    let lat_code = LatCode::from_degrees(body.lat);
    let lon_code = LonCode::from_degrees(body.lon);

    let uvox = UvoxId::new(RUm(r_um_i64), lat_code, lon_code);

    Json(json!({
        "status": "ok",
        "uvoxid": {
            "r_um": uvox.r_um.0,
            "lat_code": uvox.lat_code.0,
            "lon_code": uvox.lon_code.0,
            "hex": uvox.to_hex(),
        }
    }))
}

//
// ------------------------
// 2. POST /uvox/to_coords
// ------------------------
// Convert UVoxId → Earth lat/lon/elevation
//

#[derive(Debug, Deserialize)]
pub struct UvoxToCoordsPayload {
    pub r_um: i64,
    pub lat_code: i64,
    pub lon_code: i64,
}

#[derive(Debug, Serialize)]
pub struct CoordsResponse {
    pub lat: f64,
    pub lon: f64,
    pub elevation_m: f64,
}

pub async fn uvox_to_coords(
    State(_): State<AppState>,
    Json(body): Json<UvoxToCoordsPayload>,
) -> impl IntoResponse {

    let r_um = RUm(body.r_um);
    let lat_code = LatCode(body.lat_code);
    let lon_code = LonCode(body.lon_code);

    // Convert back to degrees
    let lat_deg = lat_code.degrees();
    let lon_deg = lon_code.degrees();

    // elevation = radius - earth radius
    let elevation_m =
        (r_um.0 as f64 / 1_000_000.0) - (EARTH_RADIUS_UM as f64 / 1_000_000.0);

    Json(json!({
        "status": "ok",
        "coords": {
            "lat": lat_deg,
            "lon": lon_deg,
            "elevation_m": elevation_m,
            
            
        }
    }))
}

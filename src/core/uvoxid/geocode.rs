use crate::core::uvoxid::UvoxId;

pub fn from_lat_lon(lat: f64, lon: f64, elevation_m: f64, frame_id: i64) -> UvoxId {
    const EARTH_RADIUS_M: f64 = 6_371_000.0;
    // r in micrometers
    let r_um = ((EARTH_RADIUS_M + elevation_m) * 1_000_000.0).round() as i64;

    // lat/lon scaled to 1e11 (≈1 µm linear resolution at Earth’s surface)
    let lat_code = (lat * 1e11).round() as i64;
    let lon_code = (lon * 1e11).round() as i64;

    UvoxId::new(frame_id, r_um, lat_code, lon_code)
}
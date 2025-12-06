use crate::core::uvoxid::{UvoxId, RUm, LatCode, LonCode};

pub fn from_lat_lon(lat: f64, lon: f64, elevation_m: f64) -> UvoxId {
    const EARTH_RADIUS_M: f64 = 6_371_000.0;

    let r_um = ((EARTH_RADIUS_M + elevation_m) * 1_000_000.0).round() as i64;

    // Convert degrees → scaled integer units
    let lat_code = (lat * 1e11).round() as i64;
    let lon_code = (lon * 1e11).round() as i64;

    UvoxId::new(
        RUm(r_um),
        LatCode(lat_code),
        LonCode(lon_code),
    )
}

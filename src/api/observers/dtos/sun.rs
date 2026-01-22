use serde::Serialize;

#[derive(Serialize)]
pub struct SunAnglesResponse {
    pub observer_id: u64,
    pub time_ns: i128,
    pub azimuth_deg: f64,
    pub elevation_deg: f64,
}

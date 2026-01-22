use serde::Serialize;

#[derive(Serialize)]
pub struct MoonAnglesResponse {
    pub observer_id: u64,
    pub time_ns: i128,
    pub azimuth_deg: f64,
    pub elevation_deg: f64,
    pub distance_m: f64,
}

#[derive(Serialize)]
pub struct ObserverMoonPhaseResponse {
    pub observer_id: u64,
    pub time_ns: i128,
    pub illuminated_fraction: f64,
    pub phase_angle_rad: f64,

}

use serde::Serialize;

#[derive(Serialize)]
pub struct SurfaceEnergyResponse {
    pub observer_id: u64,
    pub time_ns: i128,
    pub direct_w_m2: f64,
    pub diffuse_w_m2: f64,
    pub total_w_m2: f64,
}

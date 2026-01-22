use serde::{Deserialize, Serialize};
use crate::core::EnvironmentalSnapshot;

#[derive(Deserialize)]
pub struct ObserverSampleQuery {
    pub time_ns: Option<String>,
}

#[derive(Deserialize)]
pub struct EnvironmentalCurveQuery {
    pub start_time_ns: String,
    pub step_ns: Option<String>,
    pub samples: Option<usize>,
}

#[derive(Serialize)]
pub struct EnvironmentalSample {
    pub time_ns: i128,
    pub snapshot: EnvironmentalSnapshot,
}

#[derive(Serialize)]
pub struct EnvironmentalCurveResponse {
    pub observer_id: u64,
    pub samples: Vec<EnvironmentalSample>,
}


#[derive(Serialize)]
pub struct CameraEclipseResponse {
    pub observer_id: u64,
    pub time_ns: i128,

    pub eclipse: String, // "none" | "partial" | "annular" | "total"

    pub center_separation_rad: f64,

    pub sun_radius_rad: f64,
    pub moon_radius_rad: f64,
}

use serde::Serialize;

#[derive(Serialize)]
pub struct HorizonEvent {
    pub body: String,          // "sun" or "moon"
    pub time_ns: i128,
    pub event: String,         // "rise" or "set"
}

#[derive(Serialize)]
pub struct CameraHorizonResponse {
    pub observer_id: u64,
    pub events: Vec<HorizonEvent>,
}

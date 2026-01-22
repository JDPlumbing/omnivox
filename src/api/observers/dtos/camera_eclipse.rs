use serde::Serialize;

#[derive(Serialize)]
pub struct CameraEclipseEvent {
    pub time_ns: i128,
    pub state: String,
}

#[derive(Serialize)]
pub struct CameraEclipseTimelineResponse {
    pub observer_id: u64,
    pub events: Vec<CameraEclipseEvent>,
}

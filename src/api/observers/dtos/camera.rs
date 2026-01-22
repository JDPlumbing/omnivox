use serde::Serialize;

#[derive(Serialize)]
pub struct CameraSkyObject {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub visible: bool,
}

#[derive(Serialize)]
pub struct CameraBodyProjection {
    pub visible: bool,
    pub ndc: Option<[f64; 2]>,
    pub angular_radius_rad: f64,
}
#[derive(Serialize)]
pub struct CameraSkyProjectedResponse {
    pub observer_id: u64,
    pub time_ns: i128,
    pub sun: CameraBodyProjection,
    pub moon: CameraBodyProjection,
}

#[derive(Serialize)]
pub struct CameraSkyResponse {
    pub observer_id: u64,
    pub time_ns: i128,

    pub sun: CameraSkyObject,
    pub moon: CameraSkyObject,
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

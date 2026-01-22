use serde::Serialize;

#[derive(Serialize)]
pub struct EnuFrameDto {
    pub east: [f64; 3],
    pub north: [f64; 3],
    pub up: [f64; 3],
}

#[derive(Serialize)]
pub struct ObserverFrameResponse {
    pub observer_id: u64,
    pub time_ns: i128,
    pub origin: [f64; 3],
    pub enu: EnuFrameDto,
}

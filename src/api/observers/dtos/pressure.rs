use serde::Serialize;

#[derive(Serialize)]
pub struct PressureSampleDto {
    pub pressure_pa: f64,
}

#[derive(Serialize)]
pub struct PressureSweepSample {
    pub altitude_m: f64,
    pub pressure_pa: f64,
}

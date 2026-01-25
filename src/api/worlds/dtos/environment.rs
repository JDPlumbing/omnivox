use serde::Serialize;
use crate::core::env::medium::Medium;

#[derive(Debug, Serialize)]
pub struct EnvSampleDto {
    pub medium: Medium,
    pub density: f64,
    pub gravity_radial: f64,
    pub pressure: f64,
    pub temperature: f64,
}



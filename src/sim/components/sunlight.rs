use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunlightComponent {
    /// NEW — direct-beam irradiance factor (0–1)
    pub irradiance_factor: f64,
    pub elevation_deg: f64,
    pub azimuth_deg: f64,
    pub is_daylight: bool,

    /// COMPAT — legacy fields old systems still expect
    #[serde(default)]
    pub irradiance_w_m2: f64,

    #[serde(default)]
    pub uv_index: f64,
}


impl SunlightComponent {
    pub fn new() -> Self {
        Self {
            irradiance_factor: 0.0,
            elevation_deg: 0.0,
            azimuth_deg: 0.0,
            is_daylight: false,

            // legacy defaults
            irradiance_w_m2: 0.0,
            uv_index: 0.0,
        }
    }
}

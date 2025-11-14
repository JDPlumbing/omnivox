use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunlightComponent {
    pub irradiance_w_m2: f64,
    pub uv_index: f64, // between 0 and ~12 based on UV fraction
}

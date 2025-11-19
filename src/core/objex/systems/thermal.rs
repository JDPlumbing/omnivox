use crate::core::objex::core::Objex;
use crate::core::objex::geospec::{Dimensions, Volume, SurfaceArea};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalProps {
    pub conductivity: f32,
    pub expansion: f32,
    pub melting_point: f32,
}

pub fn derive_thermal<T: Dimensions + Volume + SurfaceArea>(obj: &Objex<T>) -> ThermalProps {
    let m = &obj.material;
    ThermalProps {
        conductivity: m.thermal_conductivity,
        expansion: m.thermal_expansion,
        melting_point: m.melting_point,
    }
}

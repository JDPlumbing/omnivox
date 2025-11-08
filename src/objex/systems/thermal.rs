use crate::objex::core::Object;
use crate::geospec::{Dimensions, Volume, SurfaceArea};

#[derive(Debug, Clone)]
pub struct ThermalProps {
    pub conductivity: f32,
    pub expansion: f32,
    pub melting_point: f32,
}

pub fn derive_thermal<T: Dimensions + Volume + SurfaceArea>(obj: &Object<T>) -> ThermalProps {
    let m = &obj.material;
    ThermalProps {
        conductivity: m.thermal_conductivity,
        expansion: m.thermal_expansion,
        melting_point: m.melting_point,
    }
}

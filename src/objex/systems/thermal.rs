use crate::objex::core::Object;
use crate::geospec::{Dimensions, Volume, SurfaceArea};

#[derive(Debug)]
pub struct ThermalProps {
    pub conductivity: f32,
    pub expansion_coeff: f32,
    pub melting_point: f32,
    pub thermal_resistance: f64,
}

pub fn derive_thermal<T: Dimensions + Volume + SurfaceArea>(
    obj: &Object<T>,
    thickness: f64,
) -> ThermalProps {
    let k = obj.material.thermal_conductivity;
    let area = obj.surface_area();
    let thermal_resistance = if k > 0.0 {
        thickness / ((k as f64) * area)
    } else {
        f64::INFINITY
    };

    ThermalProps {
        conductivity: k,
        expansion_coeff: obj.material.thermal_expansion,
        melting_point: obj.material.melting_point,
        thermal_resistance,
    }
}

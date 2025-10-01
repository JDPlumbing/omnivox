use crate::objex::core::Object;
use crate::geospec::{Dimensions, SurfaceArea, Volume};

#[derive(Debug)]
pub struct StrengthProps {
    pub tensile_strength: f32,
    pub compressive_strength: f32,
    pub failure_load: f64, // N, approximate from stress * area
}

pub fn derive_strength<T: Dimensions + SurfaceArea + Volume>(obj: &Object<T>) -> StrengthProps {
    let area = obj.surface_area();
    let tensile = obj.material.tensile_strength;
    let compressive = obj.material.compressive_strength;

    StrengthProps {
        tensile_strength: tensile,
        compressive_strength: compressive,
        failure_load: (tensile as f64) * area,
    }
}

pub fn will_fail<T: Dimensions + SurfaceArea + Volume>(obj: &Object<T>, applied_stress: f32) -> bool {
    applied_stress > obj.material.tensile_strength
}

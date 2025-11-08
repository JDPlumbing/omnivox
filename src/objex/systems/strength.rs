use crate::objex::core::Object;
use crate::geospec::{Dimensions, Volume, SurfaceArea};

#[derive(Debug, Clone)]
pub struct StrengthProps {
    pub tensile_strength: f32,
    pub compressive_strength: f32,
}

pub fn derive_strength<T: Dimensions + Volume + SurfaceArea>(obj: &Object<T>) -> StrengthProps {
    let m = &obj.material;
    StrengthProps {
        tensile_strength: m.tensile_strength,
        compressive_strength: m.compressive_strength,
    }
}

pub fn will_fail<T: Dimensions + Volume + SurfaceArea>(obj: &Object<T>, applied_stress: f32) -> bool {
    applied_stress > obj.material.tensile_strength
}

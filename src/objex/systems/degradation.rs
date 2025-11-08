use crate::objex::core::Object;
use crate::geospec::{Dimensions, Volume, SurfaceArea};

#[derive(Debug, Clone)]
pub struct DegradationProps {
    pub corrosion_resistance: f32,
    pub fatigue_resistance: f32,
    pub estimated_lifespan_cycles: f32,
    pub estimated_lifespan_years: f32,
}

pub fn derive_degradation<T: Dimensions + Volume + SurfaceArea>(obj: &Object<T>) -> DegradationProps {
    let m = &obj.material;
    DegradationProps {
        corrosion_resistance: m.corrosion_resistance,
        fatigue_resistance: m.fatigue_resistance,
        estimated_lifespan_cycles: 1e6 * m.fatigue_resistance,
        estimated_lifespan_years: 50.0 * m.corrosion_resistance,
    }
}

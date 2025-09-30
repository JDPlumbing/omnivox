use crate::objex::core::Object;
use crate::geospec::{Dimensions, Volume, SurfaceArea};

#[derive(Debug)]
pub struct DegradationProps {
    pub fatigue_resistance: f32,
    pub corrosion_resistance: f32,
    pub estimated_lifespan_cycles: f64,
    pub estimated_lifespan_years: f64,
}

pub fn derive_degradation<T: Dimensions + Volume + SurfaceArea>(
    obj: &Object<T>
) -> DegradationProps {
    let fatigue = obj.material.fatigue_resistance;
    let corrosion = obj.material.corrosion_resistance;

    // toy formulas for now
    let estimated_cycles = 1e6 * (fatigue as f64);
    let estimated_years = 50.0 * (corrosion as f64);

    DegradationProps {
        fatigue_resistance: fatigue,
        corrosion_resistance: corrosion,
        estimated_lifespan_cycles: estimated_cycles,
        estimated_lifespan_years: estimated_years,
    }
}

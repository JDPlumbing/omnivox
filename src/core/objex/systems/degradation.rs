use crate::core::objex::core::Objex;
use crate::core::objex::geospec::traits::{SurfaceArea, Volume};
use crate::core::objex::matcat::materials::props_for;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationProps {
    pub corrosion_resistance: f32,     // 0–1
    pub fatigue_resistance: f32,       // 0–1
    pub estimated_lifespan_cycles: f32,
    pub estimated_lifespan_years: f32,

    pub surface_area_m2: f64,          // geometry
    pub volume_m3: f64,                // geometry
}

/// Compute material degradation properties.
/// Everything is driven by MatCat properties, not MaterialLink.
/// Geometry included for completeness.
pub fn derive_degradation(obj: &Objex) -> DegradationProps {
    // ------------------------------
    // Material degradation data
    // ------------------------------
    let mat_id = obj.material.matcat_id;   // MatCatId (not Option)
    let mat_props = props_for(&mat_id);

    // ------------------------------
    // Geometry
    // ------------------------------
    let area   = obj.shape.surface_area();
    let volume = obj.shape.volume();

    // ------------------------------
    // Estimated degradation metrics
    // ------------------------------
    let corrosion_resistance = mat_props.corrosion_resistance;
    let fatigue_resistance   = mat_props.fatigue_resistance;

    // Simple placeholder models
    let estimated_lifespan_cycles = 1e6 * fatigue_resistance;
    let estimated_lifespan_years  = 50.0 * corrosion_resistance;

    DegradationProps {
        corrosion_resistance,
        fatigue_resistance,
        estimated_lifespan_cycles,
        estimated_lifespan_years,

        surface_area_m2: area,
        volume_m3: volume,
    }
}

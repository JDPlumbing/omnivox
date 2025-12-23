use crate::core::objex::core::Objex;
use crate::core::objex::geospec::traits::{SurfaceArea, Volume};
use crate::core::objex::matcat::materials::props_for;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrengthProps {
    pub tensile_strength_mpa: f32,      // from MatProps
    pub compressive_strength_mpa: f32,  // from MatProps
    pub surface_area_m2: f64,           // geometry
    pub volume_m3: f64,                 // geometry
}

/// Compute strength parameters for a given Objex blueprint.
/// Material strength comes from MatCat.
/// Geometry comes from Shape traits.
pub fn derive_strength(obj: &Objex) -> StrengthProps {
    // Material props (always present)
    let mat_props = props_for(&obj.material);

    // Geometry
    let area   = obj.shape.surface_area();
    let volume = obj.shape.volume();

    StrengthProps {
        tensile_strength_mpa:     mat_props.tensile_strength,
        compressive_strength_mpa: mat_props.compressive_strength,
        surface_area_m2: area,
        volume_m3:       volume,
    }
}

/// True if applied stress (MPa) exceeds tensile strength.
pub fn will_fail(obj: &Objex, applied_stress_mpa: f32) -> bool {
    let props = props_for(&obj.material);
    applied_stress_mpa > props.tensile_strength
}

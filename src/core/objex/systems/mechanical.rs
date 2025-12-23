use crate::core::objex::core::Objex;
use crate::core::objex::geospec::traits::{SurfaceArea, Volume};
use crate::core::objex::matcat::materials::props_for;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MechanicalProps {
    pub youngs_modulus_gpa: f32,      // elastic_modulus
    pub hardness_mohs: f32,           // hardness
    pub fracture_toughness: f32,      // MPa·m^0.5
    pub inertia: f64,                 // kg·m²
}

/// Compute mechanical properties for an Objex.
/// Uses MatCat for material and shape traits for geometry.
pub fn derive_mechanical(obj: &Objex) -> MechanicalProps {

    // -------------------------------
    // Material properties
    // -------------------------------
    let mat = props_for(&obj.material);  // <-- correct fix

    // -------------------------------
    // Geometry
    // -------------------------------
    let volume_m3 = obj.shape.volume();         // m³
    let area_m2   = obj.shape.surface_area();   // m²

    let mass_kg = (mat.density as f64) * volume_m3;

    // Characteristic size = volume / area
    let char_len_m = (volume_m3 / area_m2).abs().max(1e-6);

    // Approx rotational inertia
    let inertia = mass_kg * char_len_m.powi(2);

    MechanicalProps {
        youngs_modulus_gpa: mat.elastic_modulus,
        hardness_mohs: mat.hardness,
        fracture_toughness: mat.fracture_toughness,
        inertia,
    }
}

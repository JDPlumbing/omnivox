use crate::core::objex::core::Objex;
use crate::core::objex::geospec::traits::{SurfaceArea, Volume};
use crate::core::objex::matcat::materials::props_for;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassProps {
    pub mass_kg: f64,
    pub density_kg_m3: f64,
    pub surface_area_m2: f64,
    pub volume_m3: f64,
}

/// Compute mass from Objex blueprint.
/// Uses shape geometry and MatCat-derived density.
pub fn derive_mass(obj: &Objex) -> MassProps {

    // -------------------------
    // Material density (kg/m³)
    // -------------------------
    let mat_props = props_for(&obj.material.matcat_id);
    let density   = mat_props.density as f64;

    // -------------------------
    // Geometry (from shape)
    // -------------------------
    let volume = obj.shape.volume();       // m³
    let area   = obj.shape.surface_area(); // m²

    // -------------------------
    // Mass (kg)
    // -------------------------
    let mass = volume * density;

    MassProps {
        mass_kg: mass,
        density_kg_m3: density,
        surface_area_m2: area,
        volume_m3: volume,
    }
}

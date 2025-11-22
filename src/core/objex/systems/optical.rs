use crate::core::objex::core::Objex;
use crate::core::objex::geospec::traits::{SurfaceArea, Volume};
use crate::core::objex::matcat::materials::props_for;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpticalProps {
    pub refractive_index: f32,   // 1.0–2.5
    pub reflectivity: f32,       // 0–1
    pub absorption: f32,         // 0–1

    pub surface_area_m2: f64,    // geometry
    pub volume_m3: f64,
}

/// Compute optical parameters for an Objex.
/// Uses MatCat for material physics, plus geometric context.
pub fn derive_optical(obj: &Objex) -> OpticalProps {

    // -------------------------------
    // Material optical properties
    // -------------------------------
    let mat = props_for(&obj.material.matcat_id);

    // -------------------------------
    // Geometry
    // -------------------------------
    let area   = obj.shape.surface_area();
    let volume = obj.shape.volume();

    OpticalProps {
        refractive_index: mat.refractive_index,
        reflectivity:     mat.reflectivity,
        absorption:       mat.absorption,

        surface_area_m2: area,
        volume_m3:       volume,
    }
}

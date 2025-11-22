use crate::core::objex::core::Objex;
use crate::core::objex::geospec::traits::{SurfaceArea, Volume};
use crate::core::objex::matcat::materials::props_for;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalProps {
    pub thermal_conductivity: f32,  // W/m·K
    pub thermal_expansion: f32,     // 1/K
    pub melting_point_c: f32,       // °C
    pub specific_heat_j_kg_k: f32,  // J/kg·K

    pub surface_area_m2: f64,       // m² (from shape)
    pub volume_m3: f64,             // m³ (from shape)
}

/// Derive thermal properties for a blueprint.
/// Uses:
///  - geometry from Objex.shape
///  - material thermal parameters from MatCat
pub fn derive_thermal(obj: &Objex) -> ThermalProps {

    // ----------------------------
    // Material properties
    // ----------------------------
    let mat_props = props_for(&obj.material.matcat_id);

    // ----------------------------
    // Geometry
    // ----------------------------
    let area   = obj.shape.surface_area();
    let volume = obj.shape.volume();

    ThermalProps {
        thermal_conductivity: mat_props.thermal_conductivity,
        thermal_expansion:    mat_props.thermal_expansion,
        melting_point_c:      mat_props.melting_point,
        specific_heat_j_kg_k: mat_props.specific_heat,

        surface_area_m2: area,
        volume_m3:       volume,
    }
}

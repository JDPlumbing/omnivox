use crate::core::objex::core::Objex;
use crate::core::objex::geospec::traits::{SurfaceArea, Volume};
use crate::core::objex::matcat::materials::props_for;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricalProps {
    pub conductivity: f32,   // normalized 0–1
    pub resistivity: f32,    // 1 / conductivity
    pub resistance_ohm: f64, // R = ρ L / A
    pub capacitance_f: f64,  // C = ε A / d
}

/// Derive electrical behavior from Objex blueprint.
pub fn derive_electrical(obj: &Objex) -> ElectricalProps {

    // --------------------------
    // Material electrical props
    // --------------------------
    let mat_props = props_for(&obj.material);

    let conductivity = mat_props.electrical_conductivity; 
    let resistivity = if conductivity > 0.0 {
        1.0 / conductivity
    } else {
        f32::INFINITY
    };

    // --------------------------
    // Geometry
    // --------------------------
    let volume_m3 = obj.shape.volume();
    let area_m2   = obj.shape.surface_area();

    // Characteristic length
    let length_m = volume_m3.cbrt();

    // Effective cross-sectional area
    let area_eff = (area_m2 / 6.0).max(1e-12);

    // --------------------------
    // Electrical behaviors
    // --------------------------
    // R = ρ L / A
    let resistance_ohm = (resistivity as f64) * length_m / area_eff;

    // C = ε A / d
    const EPSILON_0: f64 = 8.854e-12;
    let capacitance_f = EPSILON_0 * area_eff / length_m.max(1e-12);

    ElectricalProps {
        conductivity,
        resistivity,
        resistance_ohm,
        capacitance_f,
    }
}

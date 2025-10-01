// src/systems/electrical.rs
use crate::objex::core::Object;



#[derive(Debug, Clone)]
pub struct ElectricalProps {
    pub conductivity: f32,   // Siemens per meter
    pub resistivity: f32,    // Ω·m
    pub resistance: f32,     // Ω (across length)
    pub capacitance: f32,    // F (approx, placeholder)
}

use crate::geospec::{Dimensions, Volume, SurfaceArea};

pub fn derive_electrical<T: Dimensions + Volume + SurfaceArea>(
    obj: &Object<T>
) -> ElectricalProps {
    let mat = &obj.material;
    let vol = obj.shape.volume() as f32;

    let length = vol.cbrt();
    let area = obj.shape.surface_area() as f32 / 6.0; // crude average face area

    let conductivity = mat.electrical_conductivity;
    let resistivity = if conductivity > 0.0 { 1.0 / conductivity } else { f32::INFINITY };

    let resistance = resistivity * length / area.max(1e-6);
    let capacitance = (area / length.max(1e-6)) * 8.85e-12;

    ElectricalProps {
        conductivity,
        resistivity,
        resistance,
        capacitance,
    }
}

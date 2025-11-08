use crate::objex::core::Object;
use crate::geospec::{Dimensions, Volume, SurfaceArea};
use crate::matcat::materials::default_props;

#[derive(Debug, Clone)]
pub struct ElectricalProps {
    pub conductivity: f32,
    pub resistivity: f32,
    pub resistance: f32,
    pub capacitance: f32,
}

pub fn derive_electrical<T: Dimensions + Volume + SurfaceArea>(obj: &Object<T>) -> ElectricalProps {
    let mat_props = &obj.material;
    let vol = obj.shape.volume() as f32;
    let length = vol.cbrt();
    let area = obj.shape.surface_area() as f32 / 6.0;

    let conductivity = mat_props.electrical_conductivity;
    let resistivity = if conductivity > 0.0 { 1.0 / conductivity } else { f32::INFINITY };
    let resistance = resistivity * length / area.max(1e-6);
    let capacitance = (area / length.max(1e-6)) * 8.85e-12;

    ElectricalProps { conductivity, resistivity, resistance, capacitance }
}

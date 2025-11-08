use crate::objex::core::Object;
use crate::geospec::{Dimensions, Volume, SurfaceArea};

#[derive(Debug, Clone)]
pub struct MechanicalProps {
    pub youngs_modulus: f32,
    pub hardness: f32,
    pub fracture_toughness: f32,
    pub inertia: f64,
}

pub fn derive_mechanical<T: Dimensions + Volume + SurfaceArea>(obj: &Object<T>) -> MechanicalProps {
    let m = &obj.material;
    let vol = obj.shape.volume();
    let area = obj.shape.surface_area();
    let mass = vol * m.density as f64;
    let char_len = (vol / area).abs().max(1e-6);

    MechanicalProps {
        youngs_modulus: m.elastic_modulus,
        hardness: m.hardness,
        fracture_toughness: m.fracture_toughness,
        inertia: mass * char_len.powi(2),
    }
}

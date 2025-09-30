use crate::objex::core::Object;
use crate::geospec::{Dimensions, Volume, SurfaceArea};

/// Elasticity, hardness, fracture toughness, inertia
#[derive(Debug)]
pub struct MechanicalProps {
    pub youngs_modulus: f32,     // stiffness (Pa)
    pub hardness: f32,           // Mohs-like 0–10
    pub fracture_toughness: f32, // MPa·m^0.5
    pub inertia: f64,            // rotational inertia proxy (kg·m²)
}

/// Derive intrinsic mechanical properties.
pub fn derive_mechanical<T: Dimensions + Volume + SurfaceArea>(obj: &Object<T>) -> MechanicalProps {
    let mat = &obj.material;

    let vol = obj.shape.volume();
    let area = obj.shape.surface_area();
    let char_len = (vol / area).abs().max(1e-6);
    let mass = vol * mat.density as f64;

    MechanicalProps {
        youngs_modulus: mat.elastic_modulus,
        hardness: mat.hardness,
        fracture_toughness: mat.fracture_toughness,
        inertia: mass * char_len.powi(2),
    }
}

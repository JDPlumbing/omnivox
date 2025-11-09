use crate::objex::core::Object;
use crate::geospec::{Dimensions, Volume, SurfaceArea};
use crate::matcat::materials::default_props;
use crate::objex::core::types::Objex;

#[derive(Debug, Clone)]
pub struct MassProps {
    pub mass: f64,
    pub density: f64,
    pub surface_area_m2: f64, // ✅ new field
}

// ✅ derive from Objex wrapper
pub fn derive_mass_from_objex(obj: &Objex) -> MassProps {
    let volume = obj.shape.volume();
    let surface_area = obj.shape.surface_area();
    let density = obj.material.props().map(|p| p.density as f64).unwrap_or(0.0);
    let mass = volume * density;

    MassProps {
        mass,
        density,
        surface_area_m2: surface_area,
    }
}

// ✅ derive directly from Object<T>
pub fn derive_mass<T: Dimensions + Volume + SurfaceArea>(obj: &Object<T>) -> MassProps {
    let vol = obj.shape.volume();
    let density = obj.material.density as f64;
    let mass = vol * density;
    let area = obj.shape.surface_area();

    MassProps {
        mass,
        density,
        surface_area_m2: area,
    }
}

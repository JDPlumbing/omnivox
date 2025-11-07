use crate::objex::core::{Objex, Object, Shape}; // ✅ bring in Objex + Shape
use crate::geospec::{Dimensions, Volume, SurfaceArea};

#[derive(Debug)]
pub struct MassProps {
    pub mass: f64,
    pub density: f32,
    pub volume: f64,
    pub surface_area: f64,
    pub surface_to_volume: f64,
}

pub fn derive_mass<T: Dimensions + Volume + SurfaceArea>(obj: &Object<T>) -> MassProps {
    let volume = obj.volume();
    let surface_area = obj.surface_area();
    let density = obj.material.density;
    let mass = density as f64 * volume;

    MassProps {
        mass,
        density,
        volume,
        surface_area,
        surface_to_volume: surface_area / volume,
    }
}

/// Wrapper for untyped Objex → computes mass by dispatching on Shape.
pub fn derive_mass_from_objex(obj: &Objex) -> f64 {
    match &obj.shape {
        Shape::Box(b) => {
            let mat_id = obj.material.matcat_id.unwrap_or_default();
            derive_mass(&Object::new(b.clone(), mat_id)).mass
        }
        Shape::Sphere(s) => {
            let mat_id = obj.material.matcat_id.unwrap_or_default();
            derive_mass(&Object::new(s.clone(), mat_id)).mass
        }
        Shape::Cylinder(c) => {
            let mat_id = obj.material.matcat_id.unwrap_or_default();
            derive_mass(&Object::new(c.clone(), mat_id)).mass
        }
        _ => 0.0, // fallback for shapes not yet handled
    }
}

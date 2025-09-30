use crate::objex::core::Object;
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

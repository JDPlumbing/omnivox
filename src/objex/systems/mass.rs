use crate::objex::core::Object;
use crate::geospec::{Dimensions, Volume, SurfaceArea};
use crate::matcat::materials::default_props;

#[derive(Debug, Clone)]
pub struct MassProps {
    pub mass: f64,
    pub density: f32,
}

pub fn derive_mass<T: Dimensions + Volume + SurfaceArea>(obj: &Object<T>) -> MassProps {
    let vol = obj.shape.volume();
    let density = obj.material.density;
    let mass = vol * density as f64;
    MassProps { mass, density }
}

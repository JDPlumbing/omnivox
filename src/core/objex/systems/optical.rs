use crate::core::objex::core::Object;
use crate::core::objex::geospec::{Dimensions, Volume, SurfaceArea};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpticalProps {
    pub refractive_index: f32,
    pub reflectivity: f32,
    pub absorption: f32,
}

pub fn derive_optical<T: Dimensions + Volume + SurfaceArea>(obj: &Object<T>) -> OpticalProps {
    let m = &obj.material;
    OpticalProps {
        refractive_index: m.refractive_index,
        reflectivity: m.reflectivity,
        absorption: m.absorption,
    }
}

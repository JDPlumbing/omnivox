use crate::objex::core::Object;
use crate::geospec::{Dimensions, Volume, SurfaceArea};
/// Composite object made of multiple layers of different materials
pub struct CompositeObject<T: Dimensions + Volume + SurfaceArea> {
    pub layers: Vec<Object<T>>,
}

impl<T: Dimensions + Volume + SurfaceArea> CompositeObject<T> {
    pub fn new(layers: Vec<Object<T>>) -> Self {
        CompositeObject { layers }
    }

    pub fn total_mass(&self) -> f64 {
        self.layers.iter()
            .map(|obj| obj.material.density as f64 * obj.shape.volume())
            .sum()
    }

    pub fn average_conductivity(&self) -> f32 {
        let n = self.layers.len() as f32;
        self.layers.iter().map(|obj| obj.material.thermal_conductivity).sum::<f32>() / n
    }

    pub fn weakest_strength(&self) -> f32 {
        self.layers.iter()
            .map(|obj| obj.material.tensile_strength)
            .fold(f32::INFINITY, f32::min)
    }
}

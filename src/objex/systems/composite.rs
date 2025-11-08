use crate::objex::core::Object;
use crate::geospec::{Dimensions, Volume, SurfaceArea};

#[derive(Debug, Clone)]
pub struct CompositeProps {
    pub effective_density: f32,
    pub effective_strength: f32,
}

pub struct CompositeObject<T: Dimensions + Volume + SurfaceArea> {
    pub layers: Vec<Object<T>>,
}

impl<T: Dimensions + Volume + SurfaceArea> CompositeObject<T> {
    pub fn new(layers: Vec<Object<T>>) -> Self {
        Self { layers }
    }

    pub fn total_mass(&self) -> f64 {
        self.layers
            .iter()
            .map(|obj| obj.material.density as f64 * obj.shape.volume())
            .sum()
    }

    pub fn average_conductivity(&self) -> f32 {
        let n = self.layers.len() as f32;
        if n == 0.0 {
            return 0.0;
        }
        self.layers
            .iter()
            .map(|obj| obj.material.thermal_conductivity)
            .sum::<f32>()
            / n
    }

    pub fn weakest_strength(&self) -> f32 {
        self.layers
            .iter()
            .map(|obj| obj.material.tensile_strength)
            .fold(f32::INFINITY, f32::min)
    }
}

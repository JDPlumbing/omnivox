use crate::objex::core::Object;
use crate::geospec::{Dimensions, Volume, SurfaceArea};

#[derive(Debug, Clone)]
pub struct CompositeProps {
    pub effective_density: f32,
    pub effective_strength: f32,
}

/// Composite object made of multiple layers of different materials
pub struct CompositeObject<T: Dimensions + Volume + SurfaceArea> {
    pub layers: Vec<Object<T>>,
}

impl CompositeProps {
    pub fn total_mass(&self) -> f64 {
        // temporary stub until you wire real composites up
        self.effective_density as f64
    }
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


use crate::core::objex::core::Objex;
use crate::core::objex::matcat::materials::props_for;
use crate::core::objex::geospec::traits::{SurfaceArea, Volume};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeProps {
    pub effective_density: f32,
    pub effective_strength: f32,
}

/// Composite of multiple Objex layers (laminate, coating, sandwich panel, etc.)
pub struct CompositeObject {
    pub layers: Vec<Objex>,
}

impl CompositeObject {
    pub fn new(layers: Vec<Objex>) -> Self {
        Self { layers }
    }

    /// Total mass across all layers
    pub fn total_mass(&self) -> f64 {
        self.layers
            .iter()
            .map(|obj| {
                let mat_id = obj.material; // ✅ FIXED
                let props = props_for(&mat_id);

                let density = props.density as f64;
                let volume  = obj.shape.volume();

                density * volume
            })
            .sum()
    }

    /// Average thermal conductivity across all layers
    pub fn average_conductivity(&self) -> f32 {
        if self.layers.is_empty() {
            return 0.0;
        }

        let sum: f32 = self.layers
            .iter()
            .map(|obj| {
                let mat_id = obj.material; // ✅ FIXED
                let props = props_for(&mat_id);

                props.thermal_conductivity
            })
            .sum();

        sum / self.layers.len() as f32
    }

    /// Returns the weakest (minimum) tensile strength across all layers
    pub fn weakest_strength(&self) -> f32 {
        self.layers
            .iter()
            .map(|obj| {
                let mat_id = obj.material; // ✅ FIXED
                let props = props_for(&mat_id);

                props.tensile_strength
            })
            .fold(f32::INFINITY, f32::min)
    }

    /// Compute combined effective composite properties
    pub fn derive_composite_props(&self) -> CompositeProps {
        // total density = (total mass / total volume)
        let total_volume: f32 = self.layers
            .iter()
            .map(|o| o.shape.volume() as f32)
            .sum();

        let density = if total_volume > 0.0 {
            (self.total_mass() as f32) / total_volume
        } else {
            0.0
        };

        let strength = self.weakest_strength();

        CompositeProps {
            effective_density: density,
            effective_strength: strength,
        }
    }
}

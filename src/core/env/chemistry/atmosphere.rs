use std::collections::HashMap;

use crate::core::env::fields::FieldSample;
use super::{
    composition::GasComposition,
    species::Species,
};

#[derive(Debug, Clone)]
pub struct ChemistrySample {
    pub partial_pressure_pa: HashMap<Species, f64>,
    pub mass_density_kg_m3: HashMap<Species, f64>,
}

pub struct AtmosphereChemistry {
    pub composition: GasComposition,
}

impl AtmosphereChemistry {
    pub fn earth_like() -> Self {
        Self {
            composition: GasComposition::earth_like(),
        }
    }

    pub fn sample(&self, env: &FieldSample) -> ChemistrySample {
        let mut partial_pressure = HashMap::new();
        let mut mass_density = HashMap::new();

        let total_pressure = env.pressure;
        let total_density = env.density;

        for (species, fraction) in &self.composition.fractions {
            let pp = fraction * total_pressure;
            let rho = fraction * total_density;

            partial_pressure.insert(*species, pp);
            mass_density.insert(*species, rho);
        }

        ChemistrySample {
            partial_pressure_pa: partial_pressure,
            mass_density_kg_m3: mass_density,
        }
    }
}

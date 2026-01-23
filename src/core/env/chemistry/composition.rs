use std::collections::HashMap;
use super::species::Species;

#[derive(Debug, Clone)]
pub struct GasComposition {
    /// Volume (mole) fractions, must sum to 1.0
    pub fractions: HashMap<Species, f64>,
}

impl GasComposition {
    pub fn earth_like() -> Self {
        use Species::*;

        let mut fractions = HashMap::new();
        fractions.insert(Nitrogen, 0.78084);
        fractions.insert(Oxygen, 0.20946);
        fractions.insert(Argon, 0.00934);
        fractions.insert(CarbonDioxide, 0.00042);
        fractions.insert(WaterVapor, 0.0); // variable later

        Self { fractions }
    }
}

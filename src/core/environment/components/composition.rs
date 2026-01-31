use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasFraction {
    /// Name or identifier of the gas (e.g. "N2", "O2", "CO2")
    pub name: String,

    /// Fraction by moles (should sum to ~1.0)
    pub molar_fraction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphericComposition {
    pub gases: Vec<GasFraction>,
}

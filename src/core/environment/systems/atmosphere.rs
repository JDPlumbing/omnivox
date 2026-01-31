use crate::core::environment::components::composition::{
    AtmosphericComposition,
};
use crate::core::physics::units::mass::Kilograms;

/// Mean molecular weight in kg/mol
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeanMolecularWeight(pub Kilograms);

/// Compute mean molecular weight from atmospheric composition.
/// Assumes molar fractions sum to ~1.0.
pub fn mean_molecular_weight(
    composition: &AtmosphericComposition,
) -> MeanMolecularWeight {
    let mut mu = 0.0;

    for gas in &composition.gases {
        let molar_mass = molar_mass_kg_per_mol(&gas.name);
        mu += gas.molar_fraction * molar_mass;
    }

    MeanMolecularWeight(Kilograms(mu))
}

/// Minimal molar mass lookup (kg/mol).
/// This is intentionally small and expandable.
fn molar_mass_kg_per_mol(gas: &str) -> f64 {
    match gas {
        "N2" => 28.0134e-3,
        "O2" => 31.9988e-3,
        "CO2" => 44.0095e-3,
        "Ar" => 39.948e-3,
        "H2O" => 18.01528e-3,
        _ => panic!("Unknown gas '{}'", gas),
    }
}

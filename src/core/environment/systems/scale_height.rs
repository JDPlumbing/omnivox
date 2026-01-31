use crate::core::physics::constants::GAS_CONSTANT;
use crate::core::physics::units::{
    length::Meters,
    temperature::Kelvin,
    
    acceleration::MetersPerSecondSquared,
};
use crate::core::environment::systems::atmosphere::MeanMolecularWeight;

/// Compute atmospheric scale height.
///
/// H = (R * T) / (μ * g)

pub fn scale_height(
    temperature: Kelvin,
    mean_molecular_weight: MeanMolecularWeight,
    gravity: MetersPerSecondSquared,
) -> Meters {
    let h = (GAS_CONSTANT * temperature.0)
        / (mean_molecular_weight.0 .0 * gravity.0);

    Meters(h)
}

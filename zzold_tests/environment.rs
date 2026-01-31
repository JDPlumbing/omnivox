use crate::core::environment::systems::thermal::equilibrium_temperature;
use crate::core::physics::units::irradiance::WattsPerSquareMeter;

#[test]
fn zero_insolation_gives_zero_temperature() {
    let t = equilibrium_temperature(WattsPerSquareMeter(0.0), 0.3);
    assert_eq!(t.0, 0.0);
}

#[test]
fn earth_like_equilibrium_temperature_is_reasonable() {
    // Solar constant divided by 4 for global average
    let insolation = WattsPerSquareMeter(1361.0 / 4.0);

    let albedo = 0.3;

    let t = equilibrium_temperature(insolation, albedo);

    // Earth's effective temperature ≈ 255 K
    assert!(
        t.0 > 240.0 && t.0 < 270.0,
        "expected Earth-like equilibrium temperature, got {} K",
        t.0
    );
}
use crate::core::environment::components::composition::{
    AtmosphericComposition, GasFraction,
};
use crate::core::environment::systems::atmosphere::mean_molecular_weight;

#[test]
fn earth_like_mean_molecular_weight_is_reasonable() {
    let composition = AtmosphericComposition {
        gases: vec![
            GasFraction { name: "N2".into(), molar_fraction: 0.78 },
            GasFraction { name: "O2".into(), molar_fraction: 0.21 },
            GasFraction { name: "Ar".into(), molar_fraction: 0.01 },
        ],
    };

    let mu = mean_molecular_weight(&composition);

    // Earth ≈ 28.97 g/mol = 0.02897 kg/mol
    assert!(mu.0 .0 > 0.028 && mu.0 .0 < 0.030);
}

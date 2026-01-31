use crate::core::environment::systems::barometric::pressure_at_altitude;
use crate::core::physics::units::{length::Meters, pressure::Pascals};

#[test]
fn pressure_at_sea_level_is_surface_pressure() {
    let p0 = Pascals(101_325.0);
    let h = Meters(8_500.0);
    let z = Meters(0.0);

    let p = pressure_at_altitude(p0, h, z);

    assert_eq!(p.0, p0.0);
}
#[test]
fn pressure_drops_by_e_at_one_scale_height() {
    let p0 = Pascals(100_000.0);
    let h = Meters(10_000.0);
    let z = Meters(10_000.0);

    let p = pressure_at_altitude(p0, h, z);

    let expected = p0.0 / std::f64::consts::E;
    assert!((p.0 - expected).abs() < 1.0);
}
#[test]
fn pressure_approaches_zero_at_high_altitude() {
    let p0 = Pascals(100_000.0);
    let h = Meters(8_000.0);
    let z = Meters(100_000.0);

    let p = pressure_at_altitude(p0, h, z);

    assert!(p.0 < 100.0);
}
use crate::core::environment::systems::{
    atmosphere::{mean_molecular_weight},
    scale_height::scale_height,
};
use crate::core::environment::components::composition::{
    AtmosphericComposition, GasFraction,
};
use crate::core::physics::units::temperature::Kelvin;

#[test]
fn earth_like_scale_height_is_reasonable() {
    let composition = AtmosphericComposition {
        gases: vec![
            GasFraction { name: "N2".into(), molar_fraction: 0.78 },
            GasFraction { name: "O2".into(), molar_fraction: 0.21 },
            GasFraction { name: "Ar".into(), molar_fraction: 0.01 },
        ],
    };

    let mu = mean_molecular_weight(&composition);
    let temp = Kelvin(288.0);       // ~15°C
    let gravity = 9.81;             // m/s²

    let h = scale_height(temp, mu, gravity);

    // Earth ≈ 8.5 km
    assert!(h.0 > 7_000.0 && h.0 < 10_000.0);
}

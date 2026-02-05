use std::collections::HashMap;
use crate::core::worlds::id::WorldId;
use crate::core::environment::components::{
    atmosphere::AtmosphereDescriptor,
    atmospheric_composition::AtmosphericComposition,
    atmospheric_composition::GasFraction,

};
use crate::core::physics::units::{pressure::Pascals,
                                    albedo::Albedo,

};
#[derive(Default)]
pub struct EnvironmentState {
    pub atmospheres: HashMap<WorldId, AtmosphereDescriptor>,
    pub compositions: HashMap<WorldId, AtmosphericComposition>,
    //later: oceans, surfaces, land, etc... 
}

impl EnvironmentState {    pub fn demo_earth() -> Self {
        let mut env = Self::default();

        let earth_world = WorldId(1);

        env.atmospheres.insert(
            earth_world,
            AtmosphereDescriptor {
                surface_pressure: Pascals(101_325.0),
                albedo: Albedo(0.306),
            },
        );

        env.compositions.insert(
            earth_world,
            AtmosphericComposition {
                gases: vec![
                    GasFraction { name: "N2".into(), molar_fraction: 0.7808 },
                    GasFraction { name: "O2".into(), molar_fraction: 0.2095 },
                    GasFraction { name: "Ar".into(), molar_fraction: 0.0093 },
                    GasFraction { name: "CO2".into(), molar_fraction: 0.0004 },
                ],
            },
        );

        env
    }

/* //--moon not done yet - needs worlds
   pub fn demo_moon() -> Self {
    let mut env = Self::default();

    let moon_world = WorldId(2);

    env.atmospheres.insert(
        moon_world,
        AtmosphereDescriptor {
            surface_pressure: Pascals(3e-10),
            albedo: Albedo(0.12),
        },
    );

    env.compositions.insert(
        moon_world,
        AtmosphericComposition {
            gases: vec![
                GasFraction { name: "He".into(), molar_fraction: 0.40 },
                GasFraction { name: "Ne".into(), molar_fraction: 0.40 },
                GasFraction { name: "Ar".into(), molar_fraction: 0.19 },
                GasFraction { name: "H2".into(), molar_fraction: 0.01 },
            ],
        },
    );

    env
}

        */
}
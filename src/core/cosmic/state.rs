// core/cosmic/state.rs

use std::collections::HashMap;

use crate::core::cosmic::id::CosmicBodyId;
use crate::core::cosmic::components::{
    radius::Radius,
    mass::Mass,
    luminosity::Luminosity,
    rotation::Rotation,
    axial_tilt::AxialTilt,
    orbit::Orbit,
    root::Root,
    prime_meridian::PrimeMeridian,
    cosmic_metadata::CosmicMetadata,
};

use crate::core::physics::units::{length::Meters,
                            time::Seconds,
                            angle::Radians,
                            power::Watts,
                            mass::Kilograms,

                        };

#[derive(Default)]
pub struct CosmicState {
    pub roots: HashMap<CosmicBodyId, Root>,
    
    pub luminosities: HashMap<CosmicBodyId, Luminosity>,
    pub radii: HashMap<CosmicBodyId, Radius>,
    pub masses: HashMap<CosmicBodyId, Mass>,
    pub metadatas: HashMap<CosmicBodyId, CosmicMetadata>,
    pub axial_tilts: HashMap<CosmicBodyId, AxialTilt>,
    pub rotations: HashMap<CosmicBodyId, Rotation>,
    pub orbits: HashMap<CosmicBodyId, Orbit>,
    pub prime_meridians: HashMap<CosmicBodyId, PrimeMeridian>,

}

impl CosmicState {

    //use this for tests/bootstraps
    pub fn demo_solar_system() -> Self {
        let mut cosmic = Self::default();

        let sun   = CosmicBodyId(1);
        let earth = CosmicBodyId(2);
        let moon  = CosmicBodyId(3);
        cosmic.metadatas.insert(sun, CosmicMetadata { name: "Sun".to_string() });
        cosmic.metadatas.insert(earth, CosmicMetadata { name: "Earth".to_string() });
        cosmic.metadatas.insert(moon, CosmicMetadata { name: "Moon".to_string() });
        cosmic.roots.insert(sun, Root);
        
        cosmic.radii.insert(
            sun,
            Radius { meters: Meters(696_340_000.0) },
        );
        cosmic.radii.insert(
            earth,
            Radius { meters: Meters(6_371_000.0) },
        );
        cosmic.radii.insert(
            moon,
            Radius { meters: Meters(1_737_000.0) },
        );

        cosmic.orbits.insert(
            earth,
            Orbit {
                primary: sun,
                semi_major_axis: Meters(150_000_000_000.0),
                eccentricity: 0.0167,
                period: Seconds(365.25 * 24.0 * 3600.0),
                inclination: Radians(0.0),
                phase_at_epoch: Radians(0.0),
            },
        );

        cosmic.orbits.insert(
            moon,
            Orbit {
                primary: earth,
                semi_major_axis: Meters(384_400_000.0),
                eccentricity: 0.0549,
                period: Seconds(27.3 * 24.0 * 3600.0),
                inclination: Radians(0.08979719),
                phase_at_epoch: Radians(0.0),
            },
        );

        cosmic.rotations.insert(
            sun,
            Rotation {
                period: Seconds(25.38 * 24.0 * 3600.0), // sidereal rotation
                phase_at_epoch: Radians(0.0),
            },
        );
        cosmic.axial_tilts.insert(
            sun,
            AxialTilt {
                radians: Radians(7.25_f64.to_radians()),
                longitude: Radians(0.0),
            },
        );

        cosmic.rotations.insert(
            earth,
            Rotation {
                period: Seconds(23.9344696 * 3600.0), // sidereal day
                phase_at_epoch: Radians(0.0),
            },
        );
        cosmic.prime_meridians.insert(
            earth,
            PrimeMeridian {
                radians: Radians(0.0),
            },
        );

        cosmic.rotations.insert(
            moon,
            Rotation {
                period: Seconds(27.3 * 24.0 * 3600.0),
                phase_at_epoch: Radians(0.0),
            },
        );

        cosmic.prime_meridians.insert(
            moon,
            PrimeMeridian {
                radians: Radians(0.0),
            },
        );

        cosmic.luminosities.insert(
            sun,
            Luminosity {
                watts: Watts(3.828e26), // solar luminosity
            },
        );

        cosmic.masses.insert(
            sun,
            Mass { kg: Kilograms(1.9885e30) },
        );
        cosmic.masses.insert(
            earth,
            Mass { kg: Kilograms(5.972e24) },
        );
        cosmic.masses.insert(
            moon,
            Mass { kg: Kilograms(7.34767309e22) },
        );

        cosmic.axial_tilts.insert(
            earth,
            AxialTilt {
                radians: Radians(23.44_f64.to_radians()),
                longitude: Radians(0.0), // defines equinox direction
            },
        );


        cosmic.axial_tilts.insert(
            moon,
            AxialTilt {
                radians: Radians(6.68_f64.to_radians()),
                longitude: Radians(0.0),
            },
        );



        cosmic
    }

    pub fn empty() -> Self {
        Self::default()
    }

}

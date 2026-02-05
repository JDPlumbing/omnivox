use crate::core::cosmic::state::CosmicState;
use crate::core::cosmic::id::CosmicBodyId;
use crate::core::cosmic::components::{
    orbit::Orbit,
    rotation::Rotation,
    axial_tilt::AxialTilt,
    prime_meridian::PrimeMeridian,
    root::Root,
    mass::Mass,
    radius::Radius,
    luminosity::Luminosity,
    
};
use crate::core::physics::units::{
    length::Meters,
    time::Seconds,
    angle::Radians,
    mass::Kilograms,
    power::Watts,

};



pub struct SimpleSystem {
    pub cosmic: CosmicState,
    pub star: CosmicBodyId,
    pub planet: CosmicBodyId,
}

pub fn simple_star_planet() -> SimpleSystem {
    let mut cosmic = CosmicState::default();

    let star = CosmicBodyId(1);
    let planet = CosmicBodyId(2);

    cosmic.roots.insert(star, Root {});
    cosmic.masses.insert(star, Mass { kg: Kilograms(1.989e30) });
    cosmic.radii.insert(star, Radius { meters: Meters(1.0) });
    cosmic.luminosities.insert(star, Luminosity { watts: Watts(3.828e26) });
    cosmic.masses.insert(planet, Mass { kg: Kilograms(5.972e24) });
    cosmic.radii.insert(planet, Radius { meters: Meters(6_371_000.0) });

    cosmic.orbits.insert(planet, Orbit {
        primary: star,
        semi_major_axis: Meters(1.0),
        eccentricity: 0.0,
        period: Seconds(365.0 * 86_400.0),
        inclination: Radians(0.0),
        phase_at_epoch: Radians(0.0),
    });

    cosmic.rotations.insert(planet, Rotation {
        period: Seconds(86_400.0),
        phase_at_epoch: Radians(0.0),
    });

    cosmic.axial_tilts.insert(planet, AxialTilt { radians: Radians(0.0) , longitude: Radians(0.0) });
    cosmic.prime_meridians.insert(planet, PrimeMeridian { radians: Radians(0.0) });

    SimpleSystem { cosmic, star, planet }
}

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
};

#[derive(Default)]
pub struct CosmicState {
    pub roots: HashMap<CosmicBodyId, Root>,
    
    pub luminosities: HashMap<CosmicBodyId, Luminosity>,
    pub radii: HashMap<CosmicBodyId, Radius>,
    pub masses: HashMap<CosmicBodyId, Mass>,

    pub axial_tilts: HashMap<CosmicBodyId, AxialTilt>,
    pub rotations: HashMap<CosmicBodyId, Rotation>,
    pub orbits: HashMap<CosmicBodyId, Orbit>,
    pub prime_meridians: HashMap<CosmicBodyId, PrimeMeridian>,

}

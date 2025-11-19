pub mod velocity;
pub mod acceleration;
pub mod fracture;
pub mod corrosion;
pub mod thermal;


pub use velocity::Velocity;
pub use acceleration::Acceleration;
pub use fracture::FractureData;
pub use thermal::{ThermalData, ThermalExposure};
pub use corrosion::CorrosionData;

pub mod sunlight;
pub use sunlight::SunlightComponent;

pub mod sun_emitter;
pub use sun_emitter::SunEmitter;
pub mod solar_exposure;
pub use solar_exposure::SolarExposureData;

pub mod orbital_motion;
pub use orbital_motion::OrbitalMotion;
pub mod uv_degradation;
pub use uv_degradation::UVDegradationData;


use crate::core::objex::systems::{
    MechanicalProps,
    StrengthProps,
    ElectricalProps,
    DegradationProps,
    OpticalProps,
    CompositeProps,
};
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SimComponents {
    // basic physics
    pub velocity_components: HashMap<Uuid, Velocity>,
    pub acceleration_components: HashMap<Uuid, Acceleration>,

    // transient or derived components
    pub fracture_components: HashMap<Uuid, FractureData>,
    pub corrosion_components: HashMap<Uuid, CorrosionData>,

    // full physics/property systems
    pub mass_components: HashMap<Uuid, MassProps>,
    pub mechanical_components: HashMap<Uuid, MechanicalProps>,
    pub strength_components: HashMap<Uuid, StrengthProps>,
   
    pub electrical_components: HashMap<Uuid, ElectricalProps>,
    pub degradation_components: HashMap<Uuid, DegradationProps>,
    pub optical_components: HashMap<Uuid, OpticalProps>,
    pub composite_components: HashMap<Uuid, CompositeProps>,

        // ☀️ add this
    pub sunlight_components: HashMap<Uuid, SunlightComponent>,
    pub sun_emitter_components: HashMap<Uuid, SunEmitter>,
    
    pub solar_exposure_components: HashMap<Uuid, SolarExposureData>,
    pub uv_degradation_components: HashMap<Uuid, UVDegradationData>,
    pub thermal_components: HashMap<Uuid, ThermalData>,
    pub thermal_exposure: HashMap<Uuid, ThermalExposure>,
    pub orbital_components: HashMap<Uuid, OrbitalMotion>,

}

impl SimComponents {
    pub fn new() -> Self {
        Self {
            velocity_components: HashMap::new(),
            acceleration_components: HashMap::new(),

            mass_components: HashMap::new(),
            mechanical_components: HashMap::new(),
            strength_components: HashMap::new(),

            electrical_components: HashMap::new(),
            degradation_components: HashMap::new(),
            corrosion_components: HashMap::new(),
            fracture_components: HashMap::new(),

            optical_components: HashMap::new(),
            composite_components: HashMap::new(),

            sunlight_components: HashMap::new(),
            sun_emitter_components: HashMap::new(),
            solar_exposure_components: HashMap::new(),
            uv_degradation_components: HashMap::new(),
            thermal_components: HashMap::new(),
            thermal_exposure: HashMap::new(),

            orbital_components: HashMap::new(),
        }
    }
}

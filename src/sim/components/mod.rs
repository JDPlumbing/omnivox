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


pub mod orbital_motion;
pub use orbital_motion::OrbitalMotion;

pub mod uv_degradation;
pub use uv_degradation::UVDegradationData;

pub mod solar_radiation;
pub mod solar_exposure;
pub mod sun_damage;

pub use solar_radiation::SolarRadiation;
pub use solar_exposure::SolarExposure;
pub use sun_damage::SunDamage;

pub mod camera;
pub use camera::Camera;

pub mod quaternion;
pub use quaternion::QuaternionLocal;

use crate::core::objex::systems::mass::MassProps;
use crate::core::objex::systems::mechanical::MechanicalProps;
use crate::core::objex::systems::strength::StrengthProps;
use crate::core::objex::systems::electrical::ElectricalProps;
use crate::core::objex::systems::degradation::DegradationProps;
use crate::core::objex::systems::optical::OpticalProps;
use crate::core::objex::systems::composite::CompositeProps;
use crate::core::id::entity_id::EntityId;

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SimComponents {
    // basic physics
    pub velocity_components: HashMap<EntityId, Velocity>,
    pub acceleration_components: HashMap<EntityId, Acceleration>,

    // transient or derived components
    pub fracture_components: HashMap<EntityId, FractureData>,
    pub corrosion_components: HashMap<EntityId, CorrosionData>,
    // full physics/property systems
    pub mass_components: HashMap<EntityId, MassProps>,
    pub mechanical_components: HashMap<EntityId, MechanicalProps>,
    pub strength_components: HashMap<EntityId, StrengthProps>,
   
    pub electrical_components: HashMap<EntityId, ElectricalProps>,
    pub degradation_components: HashMap<EntityId, DegradationProps>,
    pub optical_components: HashMap<EntityId, OpticalProps>,
    pub composite_components: HashMap<EntityId, CompositeProps>,

        // ☀️ add this
    pub solar_radiation: HashMap<EntityId, SolarRadiation>,
    pub sun_damage: HashMap<EntityId, SunDamage>,

    // sunlight and sun emitter components
    //pub sunlight_components: HashMap<EntityId, SunlightComponent>,
    //pub sun_emitter_components: HashMap<EntityId, SunEmitter>,

    pub solar_exposure: HashMap<EntityId, SolarExposure>,

    //pub uv_degradation_components: HashMap<EntityId, UVDegradationData>,
    pub thermal_components: HashMap<EntityId, ThermalData>,
    pub thermal_exposure: HashMap<EntityId, ThermalExposure>,
    pub orbital_components: HashMap<EntityId, OrbitalMotion>,
    pub camera_components: HashMap<EntityId, Camera>,
    pub quaternion_local: HashMap<EntityId, QuaternionLocal>,

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

            solar_radiation: HashMap::new(),
            solar_exposure: HashMap::new(),
            sun_damage: HashMap::new(),

            //sunlight_components: HashMap::new(),
            //sun_emitter_components: HashMap::new(),


            //uv_degradation_components: HashMap::new(),
            thermal_components: HashMap::new(),
            thermal_exposure: HashMap::new(),

            orbital_components: HashMap::new(),
            camera_components: HashMap::new(),
            quaternion_local: HashMap::new(),

        }
    }
}

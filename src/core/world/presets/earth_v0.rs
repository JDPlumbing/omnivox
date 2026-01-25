use crate::core::world::world_env_descriptor::*;
use crate::core::env::medium::Medium;

pub fn earth_v0() -> WorldEnvDescriptor {
    WorldEnvDescriptor {
        space: WorldSpace {
            // Mean Earth radius
            surface_radius_m: 6_371_000.0,
            up_model: UpModel::Radial,
        },

        gravity: GravityModel {
            kind: GravityKind::Radial,
            strength: 9.80665,
        },

        medium: MediumModel {
            // Above surface defaults to gas
            default: Medium::Gas,
        },
        land: Some(LandModel::Noise),

        atmosphere: Some(AtmosphereModel {
            sea_level_density: 1.225,
            scale_height_m: 8_500.0,
            max_height_m: Some(120_000.0),
        }),

        temperature: Some(TemperatureModel {
            surface_temp_k: 288.15,
            lapse_rate_k_per_m: Some(-0.0065),
        }),

        pressure: Some(PressureModel {
            derive_from_density: true,
        }),
    }
}

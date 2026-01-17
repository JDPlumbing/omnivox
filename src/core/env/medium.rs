use crate::core::tdt::SimDuration;
use crate::core::uvoxid::UvoxId;
use crate::core::env::fields::{Field, FieldSample};
use crate::core::world::world_env_descriptor::{WorldSpace, MediumModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Medium {
    Solid,
    Liquid,
    Gas,
    Vacuum,
}


#[derive(Debug, Clone)]
pub struct MediumField {
    pub surface_radius_m: f64,
    pub sea_level_radius_m: f64,
    pub atmosphere_limit_m: f64,
}
impl Field for MediumField {
    fn sample(&self, id: &UvoxId, _time: SimDuration) -> FieldSample {
        let r = id.r_um.meters();

    let medium = if r < self.surface_radius_m {
        Medium::Solid
    } else if r < self.sea_level_radius_m {
        Medium::Liquid
    } else if r < self.atmosphere_limit_m {
        Medium::Gas
    } else {
        Medium::Vacuum
    };


        FieldSample {
            medium: medium,
            ..Default::default()
        }
    }
}

impl MediumField {
    pub fn from_space(space: &WorldSpace, model: &MediumModel) -> Self {
        Self {
            surface_radius_m: space.surface_radius_m,
            sea_level_radius_m: space.surface_radius_m, // adjust later if needed
            atmosphere_limit_m: space.surface_radius_m * 1.02, // placeholder
        }
    }
}

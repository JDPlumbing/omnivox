use crate::core::tdt::SimDuration;
use crate::core::uvoxid::UvoxId;
use crate::core::env::fields::{Field, FieldSample};
use crate::core::world::world_env_descriptor::{WorldSpace, MediumModel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::core::env::land::height_field::LandHeightField;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Medium {
    Solid,
    Liquid,
    Gas,
    Vacuum,
}



#[derive(Debug, Clone)]
pub struct MediumField {
    pub space: WorldSpace,
    pub land: Arc<dyn LandHeightField>,
    pub atmosphere_limit_m: f64,
}
impl Field for MediumField {
    fn sample(&self, _id: &UvoxId, _time: SimDuration) -> FieldSample {
        // Medium depends on land height → derived phase
        FieldSample::default()
    }

    fn derive(
        &self,
        id: &UvoxId,
        _time: SimDuration,
        env: &FieldSample,
    ) -> FieldSample {
        let z = id.r_um.meters() - self.space.surface_radius_m;
        let land_height = env.land_height_m;

        let medium = if z < land_height {
            Medium::Solid
        } else if z < 0.0 {
            Medium::Liquid
        } else if z < self.atmosphere_limit_m {
            Medium::Gas
        } else {
            Medium::Vacuum
        };

        FieldSample {
            medium,
            ..Default::default()
        }
    }
}



impl MediumField {
    pub fn from_space(
        space: &WorldSpace,
        land: Arc<dyn LandHeightField>,
        _model: &MediumModel,
    ) -> Self {
        Self {
            space: space.clone(),
            land,
            atmosphere_limit_m: space.surface_radius_m * 1.02,
        }
    }
}

impl Medium {
    #[inline]
    pub fn is_vacuum(&self) -> bool {
        matches!(self, Medium::Vacuum)
    }

    #[inline]
    pub fn is_gas(&self) -> bool {
        matches!(self, Medium::Gas)
    }

    #[inline]
    pub fn is_liquid(&self) -> bool {
        matches!(self, Medium::Liquid)
    }

    #[inline]
    pub fn is_solid(&self) -> bool {
        matches!(self, Medium::Solid)
    }
}

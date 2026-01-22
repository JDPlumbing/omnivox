use crate::core::uvoxid::UvoxId;
use crate::core::tdt::sim_duration::SimDuration;
use crate::core::env::fields::{Field, FieldSample};
use crate::core::world::world_env_descriptor::{WorldEnvDescriptor, WorldSpace};

use std::sync::Arc;

#[derive(Clone)]
pub struct WorldEnvironment {
    pub space: WorldSpace,
    pub fields: Vec<Arc<dyn Field>>,
}

impl WorldEnvironment {
    pub fn from_descriptor(desc: &WorldEnvDescriptor) -> Self {
        let mut fields: Vec<Arc<dyn Field>> = Vec::new();

        use crate::core::env::{
            gravity::GravityField,
            medium::MediumField,
            atmosphere::AtmosphereField,
        };

        fields.push(Arc::new(
            GravityField::from_model(&desc.space, &desc.gravity),
        ));

        fields.push(Arc::new(
            MediumField::from_space(&desc.space, &desc.medium),
        ));

        if let Some(atm) = &desc.atmosphere {
            fields.push(Arc::new(
                AtmosphereField::from_model(&desc.space, atm),
            ));
        }

        Self {
            space: desc.space.clone(),
            fields,
        }
    }

    pub fn sample(
        &self,
        uvox: &UvoxId,
        time: SimDuration,
    ) -> FieldSample {
        let mut out = FieldSample::default();

        for field in &self.fields {
            out.merge(field.sample(uvox, time));
        }

        out
    }

}

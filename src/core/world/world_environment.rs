use crate::core::uvoxid::UvoxId;
use crate::core::tdt::sim_duration::SimDuration;
use crate::core::env::fields::{Field, FieldSample};
use crate::core::world::world_env_descriptor::{WorldEnvDescriptor, WorldSpace, LandModel};
use crate::core::env::pressure::PressureField;
use crate::core::env::chemistry::OceanChemistry;
use std::sync::Arc;
use crate::core::env::land::models::flat::FlatLand;
use crate::core::env::land::height_field::LandHeightField;



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

    // ----------------------------------
    // Land (world-level state)
    // ----------------------------------
    let land: Arc<dyn LandHeightField> = match &desc.land {
        Some(LandModel::Flat) | None => Arc::new(FlatLand),
    };

    // ----------------------------------
    // Gravity
    // ----------------------------------
    fields.push(Arc::new(
        GravityField::from_model(&desc.space, &desc.gravity),
    ));

    // ----------------------------------
    // Medium (depends on land)
    // ----------------------------------
    fields.push(Arc::new(
        MediumField::from_space(&desc.space, land.clone(), &desc.medium),
    ));

    // ----------------------------------
    // Atmosphere
    // ----------------------------------
    let atmosphere_field = desc.atmosphere.as_ref().map(|atm| {
        Arc::new(AtmosphereField::from_model(&desc.space, atm))
    });

    if let Some(atm) = &atmosphere_field {
        fields.push(atm.clone());
    }

    // ----------------------------------
    // Pressure
    // ----------------------------------
    if let Some(atm) = atmosphere_field {
        fields.push(Arc::new(
            PressureField {
                space: desc.space.clone(),
                atmosphere: Some((*atm).clone()),
                ocean_chemistry: Some(OceanChemistry::earth_like()),
                step_m: 100.0,
            }
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

        // Phase 1: primitive fields
        for field in &self.fields {
            out.merge(field.sample(uvox, time));
        }

        // Phase 2: derived fields
        for field in &self.fields {
            out.merge(field.derive(uvox, time, &out));
        }

        out
    }

}

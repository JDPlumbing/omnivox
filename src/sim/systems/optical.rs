use crate::core::{
    objex::core::{Objex, Object},
    objex::systems::optical::{derive_optical, OpticalProps},
    
    objex::geospec::Shape,
    objex::matcat::materials::{props_for, default_props},
    chronovox::ChronoEvent,
};
use crate::sim::{systems::System, world::WorldState},
use uuid::Uuid;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OpticalSystem;

impl System for OpticalSystem {
    fn name(&self) -> &'static str {
        "optical"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        for (id, objex) in &world.objects {
            let mat = if let Some(mat_id) = &objex.material.matcat_id {
                props_for(mat_id)
            } else {
                default_props()
            };

            let object = Object {
                shape: objex.shape.clone(),
                material: mat,
            };

            let props = derive_optical(&object);
            world.components.optical_components.insert(Uuid::parse_str(id).unwrap_or_default(), props);

        }

        events
    }
}

use crate::core::{
    objex::core::{Objex},
    objex::systems::degradation::{derive_degradation, DegradationProps},
    
    chronovox::ChronoEvent,
    objex::matcat::materials::{props_for, default_props},
};
use crate::sim::{systems::System, world::WorldState},
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DegradationSystem;

impl System for DegradationSystem {
    fn name(&self) -> &'static str {
        "degradation"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        for (_id_str, objex) in &world.objects {
            let mat = objex
                .material
                .matcat_id
                .map(|id| props_for(&id))

                .unwrap_or_else(default_props);

            let object = Object {
                shape: objex.shape.clone(),
                material: mat,
            };

            let props = derive_degradation(&object);

            // FIXED: use objex.entity_id directly
            world.components
                .degradation_components
                .insert(objex.entity_id, props);
        }

        events
    }
}

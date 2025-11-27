use crate::core::{
    chronovox::ChronoEvent,
    objex::core::Objex,
    objex::systems::degradation::{derive_degradation, DegradationProps},
    objex::matcat::materials::{props_for, default_props},
};

use crate::sim::{
    systems::System,
    world::WorldState,
};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DegradationSystem;

impl System for DegradationSystem {
    fn name(&self) -> &'static str {
        "DegradationSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        for (entity_id, entity) in world.entities.iter() {

            //---------------------------------------------------------
            // Material properties (MatProps)
            //---------------------------------------------------------
            let mat_id = &entity.material().matcat_id;
            let _mat_props = props_for(mat_id);

            //---------------------------------------------------------
            // Construct Objex blueprint
            //---------------------------------------------------------
            let object = Objex {
                shape: entity.shape().clone(),
                material: entity.material().clone(),
            };

            //---------------------------------------------------------
            // Compute degradation
            //---------------------------------------------------------
            let props: DegradationProps = derive_degradation(&object);

            //---------------------------------------------------------
            // Store degradation component
            //---------------------------------------------------------
            world.components
                .degradation_components
                .insert(*entity_id, props);
        }

        events
    }
}

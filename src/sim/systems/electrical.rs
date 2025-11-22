use crate::core::{
    chronovox::ChronoEvent,
    objex::core::Objex,
    objex::systems::electrical::{derive_electrical, ElectricalProps},
    objex::matcat::materials::props_for,
};

use crate::sim::{
    systems::System,
    world::WorldState,
};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ElectricalSystem;

impl System for ElectricalSystem {
    fn name(&self) -> &'static str {
        "ElectricalSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        for (entity_id, entity) in world.entities.iter() {

            //---------------------------------------------------------
            // Material properties (MatProps)
            //---------------------------------------------------------
            let mat_id = &entity.material().matcat_id;
            let mat_props = props_for(mat_id);

            //---------------------------------------------------------
            // Construct Objex blueprint
            //---------------------------------------------------------
            let object = Objex {
                shape: entity.shape().clone(),
                material: entity.material().clone(),
            };

            //---------------------------------------------------------
            // Compute electrical properties
            //---------------------------------------------------------
            let props: ElectricalProps = derive_electrical(&object);

            //---------------------------------------------------------
            // Store component
            //---------------------------------------------------------
            world.components
                .electrical_components
                .insert(*entity_id, props);
        }

        events
    }
}

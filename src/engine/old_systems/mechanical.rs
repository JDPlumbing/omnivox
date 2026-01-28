use crate::core::{
    chronovox::ChronoEvent,
    objex::core::Objex,
    objex::systems::mechanical::{derive_mechanical, MechanicalProps},
    objex::matcat::materials::props_for,
};

use crate::engine::{
    systems::System,
    world::WorldState,
};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MechanicalSystem;

impl System for MechanicalSystem {
    fn name(&self) -> &'static str {
        "MechanicalSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        for (id, entity) in world.entities.iter() {

            //---------------------------------------------------------
            // Fetch MatProps (physics material properties)
            //---------------------------------------------------------
            let mat_id = &entity.material();
            let mat_props = props_for(mat_id);

            //---------------------------------------------------------
            // Build Objex blueprint (Shape + MaterialLink)
            //---------------------------------------------------------
            let object = Objex {
                shape: entity.shape().clone(),
                material: entity.material().clone(),
            };

            //---------------------------------------------------------
            // Derive mechanical properties
            //---------------------------------------------------------
            let props: MechanicalProps = derive_mechanical(&object);

            //---------------------------------------------------------
            // Store mechanical component
            //---------------------------------------------------------
            world.components
                .mechanical_components
                .insert(*id, props);
        }

        events
    }
}

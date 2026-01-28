use crate::core::{
    chronovox::ChronoEvent,
    objex::core::Objex,
    objex::systems::mass::{derive_mass, MassProps},
    objex::matcat::materials::props_for,
};

use crate::engine::{
    systems::System,
    world::WorldState,
};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MassSystem;

impl System for MassSystem {
    fn name(&self) -> &'static str {
        "MassSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        for (id, entity) in world.entities.iter() {

            //---------------------------------------------------------
            // Material properties (MatProps)
            //---------------------------------------------------------
            let mat_id = &entity.material();
            let mat_props = props_for(mat_id);

            //---------------------------------------------------------
            // Construct Objex blueprint
            //---------------------------------------------------------
            let object = Objex {
                shape: entity.shape().clone(),
                material: entity.material().clone(),
            };

            //---------------------------------------------------------
            // Compute mass / density
            //---------------------------------------------------------
            let props: MassProps = derive_mass(&object);

            //---------------------------------------------------------
            // Store component
            //---------------------------------------------------------
            world.components
                .mass_components
                .insert(*id, props);
        }

        events
    }
}

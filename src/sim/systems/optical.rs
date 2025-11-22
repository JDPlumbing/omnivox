use crate::core::{
    chronovox::ChronoEvent,
    objex::core::Objex,
    objex::systems::optical::{derive_optical, OpticalProps},
    objex::matcat::materials::{props_for, default_props},
};
use crate::sim::{
    systems::System,
    world::WorldState,
};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OpticalSystem;

impl System for OpticalSystem {
    fn name(&self) -> &'static str {
        "OpticalSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        for (entity_id, entity) in world.entities.iter() {
            //
            // Extract MaterialLink from the entity’s blueprint
            //
            let mat_link  = entity.material().clone();
            let shape     = entity.shape().clone();

            //
            // Build a proper Objex (blueprint)
            //
            let object = Objex {
                shape,
                material: mat_link,   // ✔ correct type
            };

            //
            // Derive optical properties
            //
            let props: OpticalProps = derive_optical(&object);

            //
            // Store optical component for this entity
            //
            world.components.optical_components.insert(*entity_id, props);
        }

        events
    }
}

use crate::{
    objex::core::{Objex, Object},
    objex::systems::mechanical::{derive_mechanical, MechanicalProps},
    sim::{systems::System, world::WorldState},
    objex::Shape,
    matcat::materials::{props_for, default_props},
    chronovox::ChronoEvent,
};
use uuid::Uuid;
pub struct MechanicalSystem;

impl System for MechanicalSystem {
    fn name(&self) -> &'static str {
        "mechanical"
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

            let props = derive_mechanical(&object);
            let uuid = objex.entity_id;
            world.mechanical_components.insert(uuid, props);



        }

        events
    }
}

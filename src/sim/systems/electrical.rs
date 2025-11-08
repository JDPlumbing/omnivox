use crate::{
    objex::core::{Objex, Object},
    objex::systems::electrical::{derive_electrical, ElectricalProps},
    sim::{systems::System, world::WorldState},
    objex::Shape,

    matcat::materials::{props_for, default_props},
    chronovox::ChronoEvent,
};
use uuid::Uuid;
pub struct ElectricalSystem;

impl System for ElectricalSystem {
    fn name(&self) -> &'static str {
        "electrical"
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

            let props = derive_electrical(&object);
            
            world.electrical_components.insert(Uuid::parse_str(id).unwrap_or_default(), props);

        }

        events
    }
}

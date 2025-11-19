use crate::core::{
    objex::core::{Objex, Object},
    objex::systems::electrical::{derive_electrical, ElectricalProps},

    chronovox::ChronoEvent,
    objex::matcat::materials::{props_for, default_props},
};
use crate::sim::{systems::System, world::WorldState},
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ElectricalSystem;

impl System for ElectricalSystem {
    fn name(&self) -> &'static str {
        "electrical"
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

            let props = derive_electrical(&object);

            // FIX: use the object’s actual UUID
            world.components.electrical_components.insert(objex.entity_id, props);
        }

        events
    }
}

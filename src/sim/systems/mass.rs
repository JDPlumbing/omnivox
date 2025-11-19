use crate::core::{
    objex::core::{Objex, Object},
    objex::systems::mass::{derive_mass, MassProps},
    objex::geospec::traits::{Dimensions, Volume, SurfaceArea},
    objex::geospec::Shape;
    objex::matcat::materials::props_for;
    chronovox::ChronoEvent,};
use crate::sim::{systems::System, world::WorldState},
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MassSystem;

impl System for MassSystem {
    fn name(&self) -> &'static str {
        "mass"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        for (id, objex) in &world.objects {
            // Convert Objex → Object<Shape>
            let mat = if let Some(mat_id) = &objex.material.matcat_id {
                props_for(mat_id)
            } else {
                crate::core::objex::matcat::materials::default_props()
            };

            let object = Object {
                shape: objex.shape.clone(),
                material: mat,
            };

            let props = derive_mass(&object);

            let uuid = objex.entity_id;
            world.components.mass_components.insert(uuid, props);


        }

        events
    }
}

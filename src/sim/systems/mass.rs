use crate::{
    objex::core::{Objex, Object},
    objex::systems::mass::{derive_mass, MassProps},
    sim::{systems::System, world::WorldState},
    geospec::traits::{Dimensions, Volume, SurfaceArea},
};
use crate::objex::Shape;
use crate::matcat::materials::props_for;
use crate::chronovox::ChronoEvent;
use uuid::Uuid;
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
                crate::matcat::materials::default_props()
            };

            let object = Object {
                shape: objex.shape.clone(),
                material: mat,
            };

            let props = derive_mass(&object);

            world.mass_components.insert(Uuid::parse_str(id).unwrap_or_default(), props);

        }

        events
    }
}

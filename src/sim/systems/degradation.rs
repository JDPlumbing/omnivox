use crate::{
    objex::core::{Objex, Object},
    objex::systems::degradation::{derive_degradation, DegradationProps},
    sim::{systems::System, world::WorldState},
    objex::Shape,

    matcat::materials::{props_for, default_props},
    chronovox::ChronoEvent,
};
use uuid::Uuid;
pub struct DegradationSystem;

impl System for DegradationSystem {
    fn name(&self) -> &'static str {
        "degradation"
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

            let props = derive_degradation(&object);
            
            world.degradation_components.insert(Uuid::parse_str(id).unwrap_or_default(), props);

        }

        events
    }
}

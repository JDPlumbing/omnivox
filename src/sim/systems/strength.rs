use crate::{
    objex::core::{Objex, Object},
    objex::systems::strength::{derive_strength, will_fail, StrengthProps},
    sim::{systems::System, world::WorldState},
    objex::Shape,
    matcat::materials::{props_for, default_props},
    chronovox::ChronoEvent,
};
use uuid::Uuid;
pub struct StrengthSystem;

impl System for StrengthSystem {
    fn name(&self) -> &'static str {
        "strength"
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

            let props = derive_strength(&object);
            let failed = will_fail(&object, 100.0);

            if failed {
                events.push(ChronoEvent::dummy());
            }
            world.strength_components.insert(Uuid::parse_str(id).unwrap_or_default(), props);


        }

        events
    }
}

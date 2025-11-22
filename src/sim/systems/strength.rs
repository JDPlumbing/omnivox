use crate::core::{
    chronovox::{ChronoEvent, EventKind},
    objex::core::Objex,
    objex::systems::strength::{derive_strength, will_fail, StrengthProps},
    objex::matcat::materials::props_for,
};

use crate::sim::{
    systems::System,
    world::WorldState,
};

use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StrengthSystem;

impl System for StrengthSystem {
    fn name(&self) -> &'static str {
        "StrengthSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        // Need sim time for events
        let Some(clock) = &world.clock else { return events };
        let now = clock.current;

        for (entity_id, entity) in world.entities.iter() {

            //---------------------------------------------------------
            // Material properties (direct — NOT Option)
            //---------------------------------------------------------
            let mat_id = &entity.material().matcat_id;
            let mat_props = props_for(mat_id);

            //---------------------------------------------------------
            // Build Objex blueprint
            //---------------------------------------------------------
            let object = Objex {
                shape: entity.shape().clone(),
                material: entity.material().clone(),
            };

            //---------------------------------------------------------
            // Compute strength model
            //---------------------------------------------------------
            let props: StrengthProps = derive_strength(&object);

            //---------------------------------------------------------
            // Store strength component
            //---------------------------------------------------------
            world.components
                .strength_components
                .insert(*entity_id, props);

            //---------------------------------------------------------
            // Check failure condition
            //---------------------------------------------------------
            let failed = will_fail(&object, 100.0);

            if failed {
                events.push(
                    ChronoEvent::new(
                        entity.entity_id,
                        entity.world_id,
                        now,
                        EventKind::Custom("StrengthFailure".into())
                    )
                );
            }
        }

        events
    }
}

use crate::chronovox::{ChronoEvent, Timeline};
use crate::chronovox::error::ChronovoxError;
use crate::chronovox::persist::insert_event_for_entity;
use crate::supabasic::Supabase;
use crate::sim::world::SimWorld;
use crate::sim::systems::System;

pub struct Simulation {
    pub world: SimWorld,
    pub systems: Vec<Box<dyn System>>,
}

impl Simulation {
    pub async fn tick(
        &mut self,
        supa: Option<&Supabase>,
    ) -> Result<Vec<ChronoEvent>, ChronovoxError> {
        self.world.current_tick += 1;

        let mut all_events = Vec::new();
        for sys in &mut self.systems {
            let events = sys.run(&mut self.world);
            all_events.extend(events);
        }

        if self.world.persist_events {
            if let Some(supa) = supa {
                for ev in &all_events {
                    if let Some(objex) = self.world.objects.get(&ev.id) {
                        insert_event_for_entity(supa, objex.entity_id, ev).await?;
                    }
                }
            }
        }

        self.world.timeline.events.extend(all_events.clone());

        Ok(all_events)
    }
}

use crate::core::chronovox::{EventKind};
use crate::engine::world::World;
use crate::core::tdt::sim_time::SimTime;
use crate::core::id::entity_id::EntityId;

pub trait AnalyticalSystem {
    /// Compute final state after dt (no events)
    fn apply_analytical(&mut self, world: &mut World, dt: SimTime);

    /// Predict the earliest event within dt
    /// Returns (time_from_now, event_kind, entity_id)
    fn predict_event(
        &self,
        world: &World,
        dt: SimTime,
    ) -> Option<(SimTime, EventKind, EntityId)>;

    /// Apply an event (state changes caused by the event)
    fn apply_event(
        &mut self,
        world: &mut World,
        entity: EntityId,
        event: &EventKind,
    );
}

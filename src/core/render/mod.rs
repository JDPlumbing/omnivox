//! Rendering is a projection layer.
//!
//! It consumes simulation state and produces visual representations.
//! It does not own time, physics, or world meaning.


pub mod adapters;
pub mod primitives;
pub mod view;
pub mod camera;
pub mod ascii;

use crate::core::render::primitives::RenderPrimitive;
use crate::core::render::view::ViewFrame;
use crate::core::simulation::state::SimulationState;
use crate::core::tdt::sim_time::SimTime;    

pub fn build_view(
    view: &ViewFrame,
    state: &SimulationState,
    time: SimTime,
) -> Vec<RenderPrimitive> {
    match view {
        ViewFrame::Cosmic => {
            adapters::cosmic::build(state, time)
        }

        ViewFrame::World { world_id } => {
            adapters::world::build(state, *world_id, time)
        }

        // Not implemented yet
        ViewFrame::Environment { .. } => Vec::new(),
        ViewFrame::Entity { .. } => Vec::new(),
    }
}

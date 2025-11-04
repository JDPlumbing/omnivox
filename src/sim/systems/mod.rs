use crate::chronovox::ChronoEvent;
use crate::sim::world::WorldState;
use crate::sim::simulation::Simulation;

pub trait System: Send + Sync {
    fn name(&self) -> &'static str;
    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent>;
}

pub mod movement;
pub use movement::*;
pub mod acceleration;
pub use acceleration::*;
pub mod collision;
pub use collision::*;
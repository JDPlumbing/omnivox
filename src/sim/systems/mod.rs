use crate::chronovox::ChronoEvent;
use crate::sim::world::World;
use crate::sim::simulation::Simulation;


pub trait System: Send + Sync {
    fn name(&self) -> &'static str;
    fn tick(&mut self, world: &mut World) -> Vec<ChronoEvent>;

}


pub mod movement;
pub use movement::*;

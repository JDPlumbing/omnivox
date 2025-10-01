use crate::chronovox::ChronoEvent;
use crate::sim::world::SimWorld;

pub trait System {
    fn run(&mut self, world: &mut SimWorld) -> Vec<ChronoEvent>;
}

pub mod movement;
pub use movement::*;

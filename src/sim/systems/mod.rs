use crate::chronovox::ChronoEvent;
use crate::sim::world::World;

pub trait System : Send{
    fn name(&self) -> &'static str;
    
    fn run(&mut self, world: &mut World) -> Vec<ChronoEvent>;

}

//pub mod movement;
//pub use movement::*;

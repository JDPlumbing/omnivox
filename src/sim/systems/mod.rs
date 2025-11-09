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
pub mod gravity;
pub use gravity::*;

pub mod fracture;
pub use fracture::*;

pub mod degradation;
pub use degradation::*;
pub mod electrical;
pub use electrical::*;
pub mod mass;
pub use mass::*;
pub mod mechanical;
pub use mechanical::*;
pub mod optical;
pub use optical::*;
pub mod composite;
pub use composite::*;
pub mod strength;
pub use strength::*;
pub mod thermal;
pub use thermal::*;
pub mod corrosion;
pub use corrosion::*;

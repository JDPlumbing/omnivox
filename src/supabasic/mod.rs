pub mod client;
pub mod error;
pub mod entities;
pub mod users;
pub mod orm;
pub mod worlds;
pub mod simulations;

pub use client::Supabase;
pub use error::{SupabasicError, Result};
pub use entities::Entity;
pub use users::User;
pub use orm::{DbModel, fetch, list, insert};

// worlds: only re-export the low-level DB functions
pub use worlds::{list_worlds, create_world};

// simulations: just re-export the model
pub use simulations::SimulationRow;

// optionally, if you want to reach into sim layer directly
pub use crate::sim::world::{World, NewWorld};

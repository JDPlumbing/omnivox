pub mod client;
pub mod error;
pub mod entities;
pub mod users;
pub mod orm;
pub mod worlds;


pub use client::Supabase;
pub use error::{SupabasicError, Result};
pub use entities::Entity;
pub use users::User;
pub use orm::{DbModel, fetch, list, insert};
pub use worlds::{list_worlds, create_world, World, NewWorld};
pub mod client;
pub mod error;

pub mod users;
pub mod orm;
pub mod worlds;
pub mod simulations;
pub mod addresses;
pub mod geolocations;
pub mod properties;
pub mod entity;

pub use entity::EntityRow;
pub use client::Supabase;
pub use error::{SupabasicError, Result};

pub use users::User;
pub use orm::{DbModel, fetch, list, insert};
pub use addresses::AddressRow;
pub use geolocations::GeolocationRecord;
// worlds: only re-export the low-level DB functions
// src/supabasic/mod.rs
pub use self::worlds::WorldRow;
pub use self::worlds::NewWorldRow;

// simulations: just re-export the model
pub use simulations::SimulationRow;

// optionally, if you want to reach into sim layer directly

pub use crate::core::World;
pub mod events;



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
pub use self::worlds::NewWorld as NewWorldRow;

// simulations: just re-export the model
pub use simulations::SimulationRow;

// optionally, if you want to reach into sim layer directly

pub use crate::engine::World;
pub mod events;

use crate::supabasic::{properties::PropertyRecord, events::EventRow};
use uuid::Uuid;

impl Supabase {
    pub async fn get_property_by_frame(&self, frame_id: i64) -> anyhow::Result<PropertyRecord> {
        let res = self
            .from("properties")
            .select("*")
            .eq("frame_id", &frame_id.to_string())

            .single()
            
            .await?;

        Ok(serde_json::from_value(res)?)
    }

    pub async fn list_entities_for_property(&self, property_id: Uuid) -> anyhow::Result<Vec<EntityRow>> {
        let res = self
            .from("objex_entities")
            .select("*")
            .eq("property_id", &property_id.to_string())
            .execute()
            .await?; // ✅ add await



        Ok(serde_json::from_value(res)?)
    }

    pub async fn list_events_for_property(&self, property_id: Uuid) -> anyhow::Result<Vec<EventRow>> {
        let res = self
            .from("events")
            .select("*")
            .eq("property_id", &property_id.to_string())
            .execute()
            .await?; // ✅ add await

        Ok(serde_json::from_value(res)?)
    }
}

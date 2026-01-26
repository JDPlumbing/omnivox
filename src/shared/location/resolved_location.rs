// shared/location/resolved_location.rs
use uuid::Uuid;
use crate::core::{WorldId, UvoxId};

#[derive(Debug, Clone, serde::Serialize)]
pub struct ResolvedLocation {
    pub address_id: Uuid,

    pub geolocation_id: Uuid,
    pub lat: f64,
    pub lon: f64,
    pub elevation_m: f64,

    pub uvox: UvoxId,
    pub world_id: WorldId,

    pub reused: bool,
}

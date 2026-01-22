use serde::{Deserialize, Serialize};
use crate::core::id::WorldId;

#[derive(Deserialize)]
pub struct CreateObserverRequest {
    pub world: WorldId,
    pub uvox: String,
}

#[derive(Serialize)]
pub struct ObserverResponse {
    pub id: u64,
    pub world: WorldId,
    pub uvox: String,
}

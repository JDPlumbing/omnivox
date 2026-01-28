use serde::Deserialize;
use crate::core::SimTime;

#[derive(Debug, Deserialize)]
pub struct SetDespawnedAtDto {
    pub time: SimTime,
}

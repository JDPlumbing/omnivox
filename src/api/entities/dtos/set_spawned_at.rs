use serde::Deserialize;
use crate::core::SimTime;

#[derive(Debug, Deserialize)]
pub struct SetSpawnedAtDto {
    pub time: SimTime,
}

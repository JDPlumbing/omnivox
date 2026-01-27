use serde::Deserialize;
use crate::core::WorldId;

#[derive(Debug, Deserialize)]
pub struct SetWorldDto {
    pub world_id: WorldId,
}

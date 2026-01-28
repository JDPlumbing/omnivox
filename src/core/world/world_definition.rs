use crate::core::{WorldId, SimTime, world::WorldEnvDescriptor};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct WorldDefinition {
    pub world_id: WorldId,
    pub name: String,
    pub description: Option<String>,
    pub world_epoch: Option<String>,
    pub environment: Option<WorldEnvDescriptor>,
}

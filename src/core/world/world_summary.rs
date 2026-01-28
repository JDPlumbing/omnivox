
use crate::core::world::world_definition::WorldDefinition;
use crate::core::id::WorldId;
use crate::core::tdt::sim_time::SimTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSummary {
    pub id: WorldId,
    pub name: String,
    pub description: Option<String>,
}

impl From<&WorldDefinition> for WorldSummary {
    fn from(def: &WorldDefinition) -> Self {
        Self {
            id: def.world_id,
            name: def.name.clone(),
            description: def.description.clone(),
            // add any other summary fields you have
        }
    }
}

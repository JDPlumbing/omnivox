use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::id::WorldId;
use crate::supabasic::entity::EntityRow;
use crate::supabasic::worlds::WorldRow;

/// A full persisted world snapshot (JSON-backed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonWorldFile {
    pub world: WorldRow,
    pub entities: Vec<EntityRow>,
}

use crate::core::WorldId;
use serde::Serialize;
use serde_json::Value;
use crate::supabasic::WorldRow;

/// DTO returned to clients
#[derive(serde::Serialize)]
pub struct WorldDto {
    pub world_id: WorldId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub environment: Value,
    pub world_epoch: Option<String>,

    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
   
}
impl WorldDto {
    pub fn from_row(row: &WorldRow) -> Self {
        Self {
            world_id: row.world_id,
            name: row.name.clone(),
            description: row.description.clone(),
            environment: row.environment
                .as_ref()
                .map(|e| serde_json::to_value(e).unwrap())
                .unwrap_or(serde_json::Value::Null),
            world_epoch: row.world_epoch.clone(),
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct WorldUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct WorldStatsDto {
    pub world_id: WorldId,
    pub entity_count: u64,
}

impl From<WorldRow> for WorldDto {
    fn from(w: WorldRow) -> Self {
        Self {
            world_id: w.world_id,
            name: w.name,
            description: w.description,
            environment: w.environment
                .map(|e| serde_json::to_value(e).unwrap())
                .unwrap_or(serde_json::Value::Null),
            world_epoch: w.world_epoch,
            created_at: w.created_at,
            updated_at: w.updated_at,
            deleted_at: w.deleted_at,
        }
    }
}

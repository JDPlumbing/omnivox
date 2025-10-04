use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::supabasic::events::EventRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    pub frame_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
     // runtime only
    #[serde(default)]
    pub events: Vec<EventRow>, // not persisted directly, populated by querying events

}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewWorld {
    pub frame_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}
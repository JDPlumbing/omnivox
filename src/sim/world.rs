use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    pub frame_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewWorld {
    pub frame_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}
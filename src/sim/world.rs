use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    pub frame_id: i64,                // signed 64-bit (Postgres bigint)
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

// Used when creating a new world (insert payload)
#[derive(Debug, Serialize, Deserialize)]
pub struct NewWorld {
    pub frame_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}


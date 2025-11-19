use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct FractureData {
    pub object_id: Uuid,
    pub plane: String,
    pub energy: f64,
    pub threshold: f32,
}

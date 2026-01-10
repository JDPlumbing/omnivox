use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::core::GeoSpec;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoSpecRecord {
    pub id: Uuid,
    pub spec: GeoSpec,
}

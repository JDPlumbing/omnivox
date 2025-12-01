use serde::{Serialize, Deserialize};
use super::uvox_region_id::UvoxRegionId;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PropertyId(pub UvoxRegionId);

impl PropertyId {
    pub fn region(&self) -> &UvoxRegionId {
        &self.0
    }
}

use serde::{Serialize, Deserialize};
use super::uvox_id::UvoxId;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UvoxRegionId {
    pub min: UvoxId,
    pub max: UvoxId,
}

impl UvoxRegionId {
    pub fn new(min: UvoxId, max: UvoxId) -> Self {
        Self { min, max }
    }

    /// Check if a coordinate lies within the region
    pub fn contains(&self, id: UvoxId) -> bool {
        id.r_um >= self.min.r_um && id.r_um <= self.max.r_um &&
        id.lat_code >= self.min.lat_code && id.lat_code <= self.max.lat_code &&
        id.lon_code >= self.min.lon_code && id.lon_code <= self.max.lon_code
    }

    /// Check region overlap
    pub fn intersects(&self, other: &Self) -> bool {
        !(other.max.r_um  < self.min.r_um  || other.min.r_um  > self.max.r_um  ||
          other.max.lat_code < self.min.lat_code || other.min.lat_code > self.max.lat_code ||
          other.max.lon_code < self.min.lon_code || other.min.lon_code > self.max.lon_code)
    }
}

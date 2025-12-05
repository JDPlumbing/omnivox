use serde::{Serialize, Deserialize};
use crate::core::uvoxid::UvoxId;

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

// ------------------------------------------------------------
// Default impl
// ------------------------------------------------------------
impl Default for UvoxRegionId {
    fn default() -> Self {
        UvoxRegionId {
            min: UvoxId::new(0, 0, 0),
            max: UvoxId::new(100, 100, 100),
        }
    }
}

impl UvoxRegionId {
    /// Convert region to a compact string based on min corner.
    pub fn to_compact_string(&self) -> String {
        format!(
            "{}_{}_{}",
            self.min.r_um,
            self.min.lat_code,
            self.min.lon_code
        )
    }

    /// Parse region from a compact string.
    /// Expected format: r_um_lat_lon
    pub fn from_compact(s: &str) -> Result<Self, anyhow::Error> {
        let parts: Vec<&str> = s.split('_').collect();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("Invalid compact region '{}'", s));
        }

        let r_um     = parts[0].parse::<i64>()?;
        let lat_code = parts[1].parse::<i64>()?;
        let lon_code = parts[2].parse::<i64>()?;

        let min = UvoxId {
            r_um,
            lat_code,
            lon_code,
        };

        // Max is undefined — simulation doesn't need full region geometry.
        // Use min for both until region math is implemented.
        let max = min;

        Ok(UvoxRegionId { min, max })
    }
}

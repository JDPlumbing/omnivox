use serde::{Serialize, Deserialize};
use crate::core::uvoxid::{UvoxId, RUm, LatCode, LonCode};

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
        id.r_um.0     >= self.min.r_um.0     && id.r_um.0     <= self.max.r_um.0 &&
        id.lat_code.0 >= self.min.lat_code.0 && id.lat_code.0 <= self.max.lat_code.0 &&
        id.lon_code.0 >= self.min.lon_code.0 && id.lon_code.0 <= self.max.lon_code.0
    }

    /// Check region overlap (AABB-style checks)
    pub fn intersects(&self, other: &Self) -> bool {
        !(other.max.r_um.0     < self.min.r_um.0     || other.min.r_um.0     > self.max.r_um.0 ||
          other.max.lat_code.0 < self.min.lat_code.0 || other.min.lat_code.0 > self.max.lat_code.0 ||
          other.max.lon_code.0 < self.min.lon_code.0 || other.min.lon_code.0 > self.max.lon_code.0)
    }
}

// ------------------------------------------------------------
// Default impl
// ------------------------------------------------------------
impl Default for UvoxRegionId {
    fn default() -> Self {
        UvoxRegionId {
            min: UvoxId::new(RUm(0), LatCode(0), LonCode(0)),
            max: UvoxId::new(RUm(100), LatCode(100), LonCode(100)),
        }
    }
}

impl UvoxRegionId {
    /// Convert region to a compact string based on min corner.
    pub fn to_compact_string(&self) -> String {
        format!(
            "{}_{}_{}",
            self.min.r_um.0,
            self.min.lat_code.0,
            self.min.lon_code.0
        )
    }

    /// Parse region from compact string: "r_um_lat_lon"
    pub fn from_compact(s: &str) -> Result<Self, anyhow::Error> {
        let parts: Vec<&str> = s.split('_').collect();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("Invalid compact region '{}'", s));
        }

        let r_um     = RUm(parts[0].parse::<i64>()?);
        let lat_code = LatCode(parts[1].parse::<i64>()?);
        let lon_code = LonCode(parts[2].parse::<i64>()?);

        let min = UvoxId::new(r_um, lat_code, lon_code);

        // For now, define max = min (simulation-specific placeholder)
        let max = min;

        Ok(UvoxRegionId { min, max })
    }
}

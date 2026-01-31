use serde::{Serialize, Deserialize};
use crate::core::uvoxid::UvoxId;
use crate::core::old_spatial::anchor::SpatialAnchor;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpatialHorizon {
    /// Center point in world coordinates
    pub center: UvoxId,

    /// Radius in micrometers (world-scale distance)
    pub radius_um: i64,
}

impl SpatialHorizon {
    pub fn new(center: UvoxId, radius_um: i64) -> anyhow::Result<Self> {
        if radius_um <= 0 {
            anyhow::bail!("SpatialHorizon radius must be positive");
        }

        Ok(Self {
            center,
            radius_um,
        })
    }

    /// Default “human-scale” horizon (~100m)
    pub fn local(center: UvoxId) -> Self {
        Self {
            center,
            radius_um: 100_000_000, // 100m in µm
        }
    }

    /// City-scale reasoning
    pub fn regional(center: UvoxId) -> Self {
        Self {
            center,
            radius_um: 10_000_000_000, // 10km
        }
    }
}

impl SpatialHorizon {
    pub fn contains(&self, point: &UvoxId) -> bool {
        self.center.approx_distance_um(point) <= self.radius_um
    }
}


impl SpatialHorizon {
    pub fn expanded(&self, factor: f64) -> anyhow::Result<Self> {
        let new_radius = (self.radius_um as f64 * factor) as i64;

        Self::new(self.center, new_radius)
    }

    pub fn with_radius(&self, radius_um: i64) -> anyhow::Result<Self> {
        Self::new(self.center, radius_um)
    }

    pub fn recentered(&self, new_center: UvoxId) -> Self {
        Self {
            center: new_center,
            radius_um: self.radius_um,
        }
    }
}

impl SpatialHorizon {
    pub fn from_anchor(
        anchor: &crate::core::old_spatial::SpatialAnchor,
        radius_um: i64,
    ) -> anyhow::Result<Self> {
        Self::new(anchor.uvox, radius_um)
    }
}

impl SpatialHorizon {
    pub fn around_anchor(anchor: &SpatialAnchor, radius_um: i64) -> Self {
        Self {
            center: anchor.uvox,
            radius_um,
        }
    }

}

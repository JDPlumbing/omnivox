use std::sync::Arc;
use anyhow::Result;

use crate::core::{WorldId, SpatialAnchor, UvoxId, SpatialHorizon};
use crate::shared::location::location_source::LocationSource;

pub struct LocationEngine {
    pub location_source: Arc<dyn LocationSource>,
}

impl LocationEngine {
    /// Resolve an address *that already exists* into a spatial anchor
    pub async fn anchor_from_address(
        &self,
        world_id: WorldId,
        address_id: uuid::Uuid,
    ) -> Result<SpatialAnchor> {
        let resolved = self
            .location_source
            .resolve_address(address_id)
            .await?;

        Ok(SpatialAnchor {
            world_id,
            uvox: resolved.uvox,   // ← extract semantic core
            address_id: Some(address_id),
        })
    }

    /// Direct spatial anchor (coords, clicks, imports, etc)
    pub fn anchor_from_uvox(
        &self,
        world_id: WorldId,
        uvox: UvoxId,
    ) -> SpatialAnchor {
        SpatialAnchor {
            world_id,
            uvox,
            address_id: None,
        }
    }

pub fn spatial_horizon_around_anchor(
    &self,
    anchor: &SpatialAnchor,
    radius_um: i64,
) -> SpatialHorizon {
    SpatialHorizon::around_anchor(anchor, radius_um)
}

}

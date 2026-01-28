use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use crate::shared::location::resolved_location::ResolvedLocation;

// shared/location/location_source.rs
#[async_trait]
pub trait LocationSource: Send + Sync {
    async fn resolve_address(
        &self,
        address_id: Uuid,
    ) -> Result<ResolvedLocation>;
    
}

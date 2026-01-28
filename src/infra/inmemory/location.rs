use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use uuid::Uuid;

use crate::shared::location::location_source::{
    LocationSource,
};
use crate::shared::location::resolved_location::ResolvedLocation;
pub struct InMemoryLocationSource {
    resolved: Mutex<HashMap<Uuid, ResolvedLocation>>,
}

impl Default for InMemoryLocationSource {
    fn default() -> Self {
        Self {
            resolved: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl LocationSource for InMemoryLocationSource {
    async fn resolve_address(
        &self,
        address_id: Uuid,
    ) -> Result<ResolvedLocation> {
        let map = self.resolved.lock().unwrap();

        map.get(&address_id)
            .cloned()
            .ok_or_else(|| anyhow!("Address not resolved"))
    }
}

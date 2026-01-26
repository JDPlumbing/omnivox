use std::sync::Arc;
use anyhow::Result;

use crate::shared::properties::property_source::PropertySource;
use crate::shared::location::location_source::LocationSource;
use crate::shared::ownership::ownership_source::OwnershipSource;

use crate::core::{
    WorldId,
    id::user_id::UserId,
    property::property::Property,
    property::property_create::CreateProperty,
};

pub struct PropertyEngine {
    pub property_source: Arc<dyn PropertySource + Send + Sync>,
    pub location_source: Arc<dyn LocationSource + Send + Sync>,
    pub ownership_source: Arc<dyn OwnershipSource + Send + Sync>,
}

impl PropertyEngine {
    pub async fn create_property(
        &self,
        actor: UserId,
        cmd: CreateProperty,
    ) -> Result<Property> {
        // 1. Create the property itself
        let property = self.property_source.create(&cmd).await?;

        // 2. Bind ownership (ENGINE decides authority)
        self.ownership_source
            .create_owner(
                actor,
                property.id,
                cmd.world_id,
            )
            .await?;

        // 3. Return canonical domain object
        Ok(property)
    }
}

use std::sync::Arc;
use anyhow::Result;

use crate::shared::properties::property_source::PropertySource;
use crate::shared::location::location_source::LocationSource;

use crate::core::{
    property::property::Property,
    property::property_create::CreateProperty,
};

pub struct PropertyEngine {
    pub property_source: Arc<dyn PropertySource + Send + Sync>,
    pub location_source: Arc<dyn LocationSource + Send + Sync>,
}

impl PropertyEngine {
    pub fn new(
        property_source: Arc<dyn PropertySource + Send + Sync>,
        location_source: Arc<dyn LocationSource + Send + Sync>,
    ) -> Self {
        Self {
            property_source,
            location_source,
        }
    }

    pub async fn create_property(
        &self,
        cmd: CreateProperty,
    ) -> Result<Property> {
        // Future invariant checks could go here:
        // - does world exist?
        // - does location exist?
        // - is location valid for world?

        let property = self.property_source.create(&cmd).await?;
        Ok(property)
    }
}

use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use uuid::Uuid;

use crate::core::WorldId;
use crate::core::property::{Property, CreateProperty, UpdateProperty};
use crate::shared::properties::property_source::PropertySource;

/// --------------------------------------------------
/// In-memory PropertySource
/// --------------------------------------------------

pub struct InMemoryPropertySource {
    properties: Mutex<HashMap<Uuid, Property>>,
}

impl Default for InMemoryPropertySource {
    fn default() -> Self {
        Self {
            properties: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl PropertySource for InMemoryPropertySource {
    // ----------------------------
    // Queries
    // ----------------------------

    async fn get(
        &self,
        property_id: Uuid,
    ) -> Result<Option<Property>> {
        let map = self.properties.lock().unwrap();
        Ok(map.get(&property_id).cloned())
    }

    async fn list_all(&self) -> Result<Vec<Property>> {
        let map = self.properties.lock().unwrap();
        Ok(map.values().cloned().collect())
    }

    async fn list_for_world(
        &self,
        world_id: WorldId,
    ) -> Result<Vec<Property>> {
        let map = self.properties.lock().unwrap();

        Ok(map
            .values()
            .filter(|p| p.world_id == world_id)
            .cloned()
            .collect())
    }

    // ----------------------------
    // Commands
    // ----------------------------

    async fn create(
        &self,
        input: &CreateProperty,
    ) -> Result<Property> {
        let mut map = self.properties.lock().unwrap();

        let property = Property {
            id: Uuid::new_v4(),
            owner_user_id: None,
            world_id: input.world_id,
            anchor: input.anchor_uvox,
            name: input.name.clone(),
        };

        map.insert(property.id, property.clone());
        Ok(property)
    }

    async fn update(
        &self,
        cmd: UpdateProperty,
    ) -> Result<Property> {
        let mut map = self.properties.lock().unwrap();

        let prop = map
            .get_mut(&cmd.property_id)
            .ok_or_else(|| anyhow!("Property not found"))?;

        // Only mutable field (for now)
        if let Some(name) = cmd.name {
            prop.name = Some(name);
        }

        Ok(prop.clone())
    }

    async fn delete(
        &self,
        property_id: Uuid,
    ) -> Result<()> {
        let mut map = self.properties.lock().unwrap();
        map.remove(&property_id);
        Ok(())
    }
}

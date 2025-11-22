use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use crate::core::uvoxid::UvoxId;
use crate::core::objex::{Objex};
use crate::core::objex::geospec::Shape;
use crate::core::objex::core::material::MaterialLink;

use crate::core::SimTime;
use crate::sim::UvoxQuat;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimEntity {
    pub entity_id: Uuid,

    /// World this entity belongs to
    pub world_id: i64,

    /// Blueprint (shape + material)
    pub blueprint: Objex,

    /// Spatial coordinates inside that world
    pub uvoxid: UvoxId,

    /// Orientation
    pub orientation: UvoxQuat,

    /// Spawn/despawn time
    pub spawned_at: SimTime,
    pub despawned_at: Option<SimTime>,

    /// UI / editor / descriptive metadata
    pub metadata: Value, // always an Object
}

impl SimEntity {
    pub fn spawn(
        blueprint: Objex,
        world_id: i64,
        uvoxid: UvoxId,
        time: SimTime,
    ) -> Self {
        Self {
            entity_id: Uuid::new_v4(),
            world_id,
            blueprint,
            uvoxid,
            orientation: UvoxQuat::identity(),
            spawned_at: time,
            despawned_at: None,
            metadata: serde_json::json!({}),
        }
    }

    pub fn despawn(&mut self, t: SimTime) {
        self.despawned_at = Some(t);
    }

    pub fn with_metadata(mut self, key: &str, value: impl Into<Value>) -> Self {
        if !self.metadata.is_object() {
            self.metadata = serde_json::json!({});
        }

        if let Some(obj) = self.metadata.as_object_mut() {
            obj.insert(key.to_string(), value.into());
        }

        self
    }

    /// Friendly helpers
    pub fn name(&self) -> &'static str {
        "ObjexInstance"
    }

    pub fn shape(&self) -> &Shape {
        &self.blueprint.shape
    }

    pub fn material(&self) -> &MaterialLink {
        &self.blueprint.material
    }
}

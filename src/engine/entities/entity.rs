use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use crate::core::uvoxid::UvoxId;
use crate::core::objex::{Objex};
use crate::core::objex::geospec::Shape;
use crate::core::objex::core::material::MaterialLink;
use crate::core::id::{EntityId, WorldId};
use crate::core::SimTime;
use crate::engine::UvoxQuat;
use crate::core::objex::matcat::materials::MatCatId;
use crate::core::tdt::sim_time::deserialize_simtime;

/// -------------------------------------------------------------------
/// In-memory representation of a simulated entity
/// -------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimEntity {
    pub id: EntityId,

    /// World this entity belongs to
    pub world_id: WorldId,

    /// objex (shape + material)
    pub template: Objex,

    /// Spatial coordinates inside that world
    pub position: UvoxId,

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
        id: EntityId,
        template: Objex,
        world_id: WorldId,
        position: UvoxId,
        orientation: UvoxQuat,
        
        spawned_at: SimTime,
    ) -> Self {
        Self {
            id: id,
            world_id,
            position: position,
            orientation: UvoxQuat::identity(),
            spawned_at: spawned_at,
            despawned_at: None,
            metadata: serde_json::json!({}),
            template,
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
        &self.template.shape
    }



    pub fn material(&self) -> &MatCatId {
        &self.template.material
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSimEntity {
    pub world_id: WorldId,
    pub template: Objex,
    pub position: UvoxId,
    pub orientation: UvoxQuat,
    #[serde(deserialize_with = "deserialize_simtime")]
    pub spawned_at: SimTime,
    pub metadata: serde_json::Value,
}
impl CreateSimEntity {
    pub fn new(
        world_id: WorldId,
        template: Objex,
        position: UvoxId,
        orientation: UvoxQuat,
        spawned_at: SimTime,
    ) -> Self {
        Self {
            world_id,
            template,
            position,
            orientation,
            spawned_at,
            metadata: serde_json::json!({}),
        }
    }
}
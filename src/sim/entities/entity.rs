use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::core::objex::Objex;        // blueprint
use crate::core::uvoxid::UvoxId;
use crate::core::tdt::{SimTime};
use crate::sim::entities::quat::UvoxQuat;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimEntity {
    pub entity_id: Uuid,
    pub blueprint: Objex,
    pub uvoxid: UvoxId,
    pub orientation: UvoxQuat,             // ← required
    pub spawned_at: SimTime,               // ← required
    pub despawned_at: Option<SimTime>,
    pub metadata: serde_json::Value,       // ← JSON
}

impl SimEntity {
    pub fn spawn(blueprint: Objex, uvoxid: UvoxId, time: SimTime) -> Self {
        Self {
            entity_id: Uuid::new_v4(),
            blueprint,
            uvoxid,
            orientation: UvoxQuat::identity(), // ← required
            spawned_at: time,                  // ← required
            despawned_at: None,
            metadata: serde_json::json!({}),   // must be Value, not Map
        }
    }

    pub fn despawn(&mut self, t: SimTime) {
        self.despawned_at = Some(t);
    }

    pub fn with_metadata(mut self, key: &str, value: impl Into<serde_json::Value>) -> Self {
        let obj = self.metadata.as_object_mut().unwrap();
        obj.insert(key.to_string(), value.into());
        self
    }
}

impl SimEntity {
    pub fn name(&self) -> &'static str {
        // Objex currently has no name; you can add one later.
        "ObjexInstance"
    }

    pub fn shape(&self) -> &Shape {
        &self.blueprint.shape
    }

    pub fn material(&self) -> &MaterialLink {
        &self.blueprint.material
    }
}

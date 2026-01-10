use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::core::objex::geospec::shape::Shape;
use crate::core::objex::geospec::traits::Dimensions;


/// ─────────────────────────────────────────────
/// Geometry templates (internal, static)
/// These are *starter* geospecs, not user-created
/// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometryTemplate {
    pub geometry_id: Uuid,
    pub label: String,
    pub shape: serde_json::Value,
}


/// Deterministic UUIDs so these remain stable forever
const GEO_BOX_ID: Uuid = Uuid::from_u128(0x1000_0000_0000_0000_0000_0000_0000_0001);
const GEO_SPHERE_ID: Uuid = Uuid::from_u128(0x1000_0000_0000_0000_0000_0000_0000_0002);


pub fn geometry_templates() -> Vec<GeometryTemplate> {
    vec![
        GeometryTemplate {
            geometry_id: GEO_BOX_ID,
            label: "Box".to_string(),
            shape: Shape::default_box().as_json(),
        },
        GeometryTemplate {
            geometry_id: GEO_SPHERE_ID,
            label: "Sphere".to_string(),
            shape: Shape::default_sphere().as_json(),
        },
    ]
}

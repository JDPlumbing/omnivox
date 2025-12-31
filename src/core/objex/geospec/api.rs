use serde::Serialize;
use crate::core::objex::geospec::Shape;
use crate::core::objex::geospec::traits::Dimensions;

#[derive(Serialize)]
pub struct GeometryTemplate {
    pub geometry_id: String,
    pub label: String,
    pub shape: serde_json::Value,
}

pub fn geometry_templates() -> Vec<GeometryTemplate> {
    vec![
        GeometryTemplate {
            geometry_id: "geo:point".into(),
            label: "Point".into(),
            shape: Shape::default_point().as_json(),
        },
        GeometryTemplate {
            geometry_id: "geo:line:1m".into(),
            label: "Line (1m)".into(),
            shape: Shape::Line(
                crate::core::objex::geospec::Line { length: 1.0 }
            ).as_json(),
        },
        GeometryTemplate {
            geometry_id: "geo:box:1x1x1".into(),
            label: "Box (1m × 1m × 1m)".into(),
            shape: Shape::default_box().as_json(),
        },
        GeometryTemplate {
            geometry_id: "geo:sphere:r1".into(),
            label: "Sphere (r=1m)".into(),
            shape: Shape::default_sphere().as_json(),
        },
        GeometryTemplate {
            geometry_id: "geo:cylinder:1x1".into(),
            label: "Cylinder (1m × 1m)".into(),
            shape: Shape::default_cylinder().as_json(),
        },
        GeometryTemplate {
            geometry_id: "geo:cone:1x1".into(),
            label: "Cone (1m × 1m)".into(),
            shape: Shape::default_cone().as_json(),
        },
    ]
}

use serde_json::Value;
use crate::core::objex::geospec::shape::Shape;
use crate::core::objex::geospec::traits::Dimensions;
use crate::core::objex::geospec::primitives::*;

pub fn infer_from_json(input: &Value) -> Option<Value> {
    match input.get("type")?.as_str()? {
        "sphere" => Some(Shape::Sphere(Sphere {
            radius: input.get("radius")?.as_f64()?,
        }).as_json()),

        "box" => Some(Shape::Box(BoxShape {
            length: input.get("length")?.as_f64()?,
            width: input.get("width")?.as_f64()?,
            height: input.get("height")?.as_f64()?,
        }).as_json()),

        "cylinder" => Some(Shape::Cylinder(Cylinder {
            radius: input.get("radius")?.as_f64()?,
            height: input.get("height")?.as_f64()?,
        }).as_json()),

        "cone" => Some(Shape::Cone(Cone {
            radius: input.get("radius")?.as_f64()?,
            height: input.get("height")?.as_f64()?,
        }).as_json()),

        "plane" => Some(Shape::Plane(Plane {
            width: input.get("width")?.as_f64()?,
            height: input.get("height")?.as_f64()?,
        }).as_json()),

        "line" => Some(Shape::Line(Line {
            length: input.get("length")?.as_f64()?,
        }).as_json()),

        "point" => Some(Shape::Point(Point).as_json()),

        _ => None,
    }
}

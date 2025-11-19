use serde_json::Value;
use crate::core::objex::geospec::shapes::{
    Shape, Sphere, BoxShape, Cylinder, Cone, Plane, Line, Point,
};
use crate::core::objex::geospec::Dimensions;

/// Attempt to infer a full canonical Shape JSON from partial input.
/// 
/// Example:
///   { "type": "sphere", "radius": 2.0 }
/// becomes:
///   { "type": "sphere", "radius": 2.0, "surface_area": ..., "volume": ... }
///
/// Returns `None` when required fields are missing.
pub fn infer_from_json(input: &Value) -> Option<Value> {
    let shape_type = input.get("type")?.as_str()?;

    match shape_type {
        "sphere" => {
            Some(Sphere {
                radius: input.get("radius")?.as_f64()?,
            }.as_json())
        }

        "box" => {
            Some(BoxShape {
                length: input.get("length")?.as_f64()?,
                width:  input.get("width")?.as_f64()?,
                height: input.get("height")?.as_f64()?,
            }.as_json())
        }

        "cylinder" => {
            Some(Cylinder {
                radius: input.get("radius")?.as_f64()?,
                height: input.get("height")?.as_f64()?,
            }.as_json())
        }

        "cone" => {
            Some(Cone {
                radius: input.get("radius")?.as_f64()?,
                height: input.get("height")?.as_f64()?,
            }.as_json())
        }

        "plane" => {
            Some(Plane {
                width:  input.get("width")?.as_f64()?,
                height: input.get("height")?.as_f64()?,
            }.as_json())
        }

        "line" => {
            Some(Line {
                length: input.get("length")?.as_f64()?,
            }.as_json())
        }

        "point" => {
            Some(Point.as_json())
        }

        _ => None,
    }
}

use serde_json::Value;
use crate::geospec::{Sphere, BoxShape, Cylinder, Cone, Plane, Line, Point, Dimensions};

/// Given partial JSON, try to infer missing dimensions.
/// Example: `{ "type": "sphere", "radius": 2.0 }` -> adds surface_area + volume.
pub fn infer_from_json(input: &Value) -> Option<Value> {
    match input.get("type")?.as_str()? {
        "sphere" => {
            let r = input.get("radius")?.as_f64()?;
            Some(Sphere { radius: r }.as_json())
        }
        "box" => {
            let l = input.get("length")?.as_f64()?;
            let w = input.get("width")?.as_f64()?;
            let h = input.get("height")?.as_f64()?;
            Some(BoxShape { length: l, width: w, height: h }.as_json())
        }
        "cylinder" => {
            let r = input.get("radius")?.as_f64()?;
            let h = input.get("height")?.as_f64()?;
            Some(Cylinder { radius: r, height: h }.as_json())
        }
        "cone" => {
            let r = input.get("radius")?.as_f64()?;
            let h = input.get("height")?.as_f64()?;
            Some(Cone { radius: r, height: h }.as_json())
        }
        "plane" => {
            let w = input.get("width")?.as_f64()?;
            let h = input.get("height")?.as_f64()?;
            Some(Plane { width: w, height: h }.as_json())
        }
        "line" => {
            let len = input.get("length")?.as_f64()?;
            Some(Line { length: len }.as_json())
        }
        "point" => {
            Some(Point.as_json())
        }
        _ => None,
    }
}

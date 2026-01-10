use serde_json::Value;

pub trait SurfaceArea {
    fn surface_area(&self) -> f64;
}

pub trait Volume {
    fn volume(&self) -> f64;
}

pub trait Dimensions {
    fn as_json(&self) -> Value;
}

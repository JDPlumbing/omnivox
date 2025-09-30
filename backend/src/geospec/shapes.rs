use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::geospec::{SurfaceArea, Volume, Dimensions};

/// A 0D object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point;

impl Dimensions for Point {
    fn as_json(&self) -> serde_json::Value {
        json!({ "type": "point" })
    }
}

/// A 1D line segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub length: f64,
}

impl Dimensions for Line {
    fn as_json(&self) -> serde_json::Value {
        json!({
            "type": "line",
            "length": self.length
        })
    }
}

/// A 2D plane
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plane {
    pub width: f64,
    pub height: f64,
}

impl SurfaceArea for Plane {
    fn surface_area(&self) -> f64 {
        self.width * self.height
    }
}

impl Dimensions for Plane {
    fn as_json(&self) -> serde_json::Value {
        json!({
            "type": "plane",
            "width": self.width,
            "height": self.height,
            "area": self.surface_area()
        })
    }
}

/// A 3D sphere
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sphere {
    pub radius: f64,
}

impl SurfaceArea for Sphere {
    fn surface_area(&self) -> f64 {
        4.0 * std::f64::consts::PI * self.radius.powi(2)
    }
}

impl Volume for Sphere {
    fn volume(&self) -> f64 {
        (4.0/3.0) * std::f64::consts::PI * self.radius.powi(3)
    }
}

impl Dimensions for Sphere {
    fn as_json(&self) -> serde_json::Value {
        json!({
            "type": "sphere",
            "radius": self.radius,
            "surface_area": self.surface_area(),
            "volume": self.volume(),
        })
    }
}

/// A 3D rectangular box (prism)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxShape {
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

impl SurfaceArea for BoxShape {
    fn surface_area(&self) -> f64 {
        2.0 * (self.length*self.width + self.length*self.height + self.width*self.height)
    }
}

impl Volume for BoxShape {
    fn volume(&self) -> f64 {
        self.length * self.width * self.height
    }
}

impl Dimensions for BoxShape {
    fn as_json(&self) -> serde_json::Value {
        json!({
            "type": "box",
            "length": self.length,
            "width": self.width,
            "height": self.height,
            "surface_area": self.surface_area(),
            "volume": self.volume(),
        })
    }
}

/// A 3D cylinder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cylinder {
    pub radius: f64,
    pub height: f64,
}

impl SurfaceArea for Cylinder {
    fn surface_area(&self) -> f64 {
        let circle_area = std::f64::consts::PI * self.radius.powi(2);
        let side_area = 2.0 * std::f64::consts::PI * self.radius * self.height;
        2.0 * circle_area + side_area
    }
}

impl Volume for Cylinder {
    fn volume(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2) * self.height
    }
}

impl Dimensions for Cylinder {
    fn as_json(&self) -> serde_json::Value {
        json!({
            "type": "cylinder",
            "radius": self.radius,
            "height": self.height,
            "surface_area": self.surface_area(),
            "volume": self.volume(),
        })
    }
}

/// A 3D cone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cone {
    pub radius: f64,
    pub height: f64,
}

impl SurfaceArea for Cone {
    fn surface_area(&self) -> f64 {
        let slant = (self.height.powi(2) + self.radius.powi(2)).sqrt();
        std::f64::consts::PI * self.radius * (self.radius + slant)
    }
}

impl Volume for Cone {
    fn volume(&self) -> f64 {
        (1.0/3.0) * std::f64::consts::PI * self.radius.powi(2) * self.height
    }
}

impl Dimensions for Cone {
    fn as_json(&self) -> serde_json::Value {
        json!({
            "type": "cone",
            "radius": self.radius,
            "height": self.height,
            "surface_area": self.surface_area(),
            "volume": self.volume(),
        })
    }
}

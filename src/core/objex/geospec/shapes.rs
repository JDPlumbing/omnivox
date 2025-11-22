use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::core::objex::geospec::{Dimensions, Volume, SurfaceArea};

/// ---------------------------------------------------------------------------
/// SHAPE ENUM
/// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shape {
    Point(Point),
    Line(Line),
    Plane(Plane),
    Sphere(Sphere),
    Box(BoxShape),
    Cylinder(Cylinder),
    Cone(Cone),
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Point(Point)
    }
}

impl Shape {
    pub fn default_point() -> Self {
        Shape::Point(Point)
    }

    pub fn default_box() -> Self {
        Shape::Box(BoxShape {
            length: 1.0,
            width: 1.0,
            height: 1.0,
        })
    }

    pub fn default_sphere() -> Self {
        Shape::Sphere(Sphere { radius: 1.0 })
    }

    pub fn default_cylinder() -> Self {
        Shape::Cylinder(Cylinder {
            radius: 1.0,
            height: 1.0,
        })
    }
}

/// ---------------------------------------------------------------------------
/// POINT (0D)
/// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point;

impl Dimensions for Point {
    fn as_json(&self) -> serde_json::Value {
        json!({ "type": "point" })
    }
}

/// ---------------------------------------------------------------------------
/// LINE (1D)
/// ---------------------------------------------------------------------------

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

/// ---------------------------------------------------------------------------
/// PLANE (2D)
/// ---------------------------------------------------------------------------

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

/// ---------------------------------------------------------------------------
/// SPHERE (3D)
/// ---------------------------------------------------------------------------

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
        (4.0 / 3.0) * std::f64::consts::PI * self.radius.powi(3)
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

/// ---------------------------------------------------------------------------
/// BOX (3D RECTANGULAR PRISM)
/// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BoxShape {
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

impl SurfaceArea for BoxShape {
    fn surface_area(&self) -> f64 {
        2.0 * (
            self.length * self.width +
            self.length * self.height +
            self.width * self.height
        )
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

/// ---------------------------------------------------------------------------
/// CYLINDER (3D)
/// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cylinder {
    pub radius: f64,
    pub height: f64,
}

impl SurfaceArea for Cylinder {
    fn surface_area(&self) -> f64 {
        let circle = std::f64::consts::PI * self.radius.powi(2);
        let side = 2.0 * std::f64::consts::PI * self.radius * self.height;
        2.0 * circle + side
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

/// ---------------------------------------------------------------------------
/// CONE (3D)
/// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cone {
    pub radius: f64,
    pub height: f64,
}

impl SurfaceArea for Cone {
    fn surface_area(&self) -> f64 {
        let slant = (self.radius.powi(2) + self.height.powi(2)).sqrt();
        std::f64::consts::PI * self.radius * (self.radius + slant)
    }
}

impl Volume for Cone {
    fn volume(&self) -> f64 {
        (1.0 / 3.0) * std::f64::consts::PI * self.radius.powi(2) * self.height
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

impl Shape {
    pub fn radius_um(&self) -> i64 {
        let r_meters = match self {
            Shape::Sphere(s) => s.radius,

            Shape::Box(b) => {
                ((b.length.powi(2) + b.width.powi(2) + b.height.powi(2)).sqrt()) / 2.0
            }

            Shape::Cylinder(c) => {
                // bounding sphere around cylinder
                let h2 = (c.height / 2.0).powi(2);
                let r2 = c.radius.powi(2);
                (h2 + r2).sqrt()
            }

            Shape::Cone(c) => {
                // bounding sphere radius for cone: distance from tip to farthest base rim point
                let slant = (c.radius.powi(2) + c.height.powi(2)).sqrt();
                slant.max(c.height) / 2.0
            }

            Shape::Plane(p) => {
                // treat plane as a thin rectangle at z=0
                ((p.width.powi(2) + p.height.powi(2)).sqrt()) / 2.0
            }

            Shape::Line(l) => l.length / 2.0,

            Shape::Point(_) => 0.0,
        };

        (r_meters * 1_000_000.0) as i64
    }
}

impl SurfaceArea for Shape {
    fn surface_area(&self) -> f64 {
        match self {
            Shape::Sphere(s)    => s.surface_area(),
            Shape::Box(b)       => b.surface_area(),
            Shape::Cylinder(c)  => c.surface_area(),
            Shape::Cone(c)      => c.surface_area(),
            Shape::Plane(p)     => p.surface_area(),
            Shape::Line(_)      => 0.0,     // 1D object
            Shape::Point(_)     => 0.0,     // 0D object
        }
    }
}

impl Volume for Shape {
    fn volume(&self) -> f64 {
        match self {
            Shape::Sphere(s)    => s.volume(),
            Shape::Box(b)       => b.volume(),
            Shape::Cylinder(c)  => c.volume(),
            Shape::Cone(c)      => c.volume(),
            Shape::Plane(_)     => 0.0, // 2D object
            Shape::Line(_)      => 0.0, // 1D object
            Shape::Point(_)     => 0.0, // 0D object
        }
    }
}

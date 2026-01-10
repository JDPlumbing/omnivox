use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::core::objex::geospec::traits::*;
use crate::core::objex::geospec::primitives::*;
use crate::core::objex::geospec::traits::Dimensions;

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
    pub fn default_box() -> Self {
        Shape::Box(BoxShape { length: 1.0, width: 1.0, height: 1.0 })
    }

    pub fn default_sphere() -> Self {
        Shape::Sphere(Sphere { radius: 1.0 })
    }

    pub fn default_cylinder() -> Self {
        Shape::Cylinder(Cylinder { radius: 1.0, height: 1.0 })
    }

    pub fn default_cone() -> Self {
        Shape::Cone(Cone { radius: 1.0, height: 1.0 })
    }
}

/* ---- TRAITS ---- */

impl SurfaceArea for Plane {
    fn surface_area(&self) -> f64 {
        self.width * self.height
    }
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

/* ---- DISPATCH ---- */

impl SurfaceArea for Shape {
    fn surface_area(&self) -> f64 {
        match self {
            Shape::Sphere(s) => s.surface_area(),
            Shape::Box(b) => b.surface_area(),
            Shape::Cylinder(c) => c.surface_area(),
            Shape::Cone(c) => c.surface_area(),
            Shape::Plane(p) => p.surface_area(),
            _ => 0.0,
        }
    }
}

impl Volume for Shape {
    fn volume(&self) -> f64 {
        match self {
            Shape::Sphere(s) => s.volume(),
            Shape::Box(b) => b.volume(),
            Shape::Cylinder(c) => c.volume(),
            Shape::Cone(c) => c.volume(),
            _ => 0.0,
        }
    }
}

impl Dimensions for Shape {
    fn as_json(&self) -> serde_json::Value {
        match self {
            Shape::Point(_) => json!({ "type": "point" }),
            Shape::Line(l) => json!({ "type": "line", "length": l.length }),
            Shape::Plane(p) => json!({ "type": "plane", "width": p.width, "height": p.height }),
            Shape::Sphere(s) => json!({ "type": "sphere", "radius": s.radius }),
            Shape::Box(b) => json!({ "type": "box", "length": b.length, "width": b.width, "height": b.height }),
            Shape::Cylinder(c) => json!({ "type": "cylinder", "radius": c.radius, "height": c.height }),
            Shape::Cone(c) => json!({ "type": "cone", "radius": c.radius, "height": c.height }),
        }
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
                let h2 = (c.height / 2.0).powi(2);
                let r2 = c.radius.powi(2);
                (h2 + r2).sqrt()
            }

            Shape::Cone(c) => {
                let slant = (c.radius.powi(2) + c.height.powi(2)).sqrt();
                slant.max(c.height) / 2.0
            }

            Shape::Plane(p) => {
                ((p.width.powi(2) + p.height.powi(2)).sqrt()) / 2.0
            }

            Shape::Line(l) => l.length / 2.0,
            Shape::Point(_) => 0.0,
        };

        (r_meters * 1_000_000.0) as i64
    }
}

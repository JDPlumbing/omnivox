use serde::{Serialize, Deserialize};
use crate::core::objex::geospec::primitives::*;
use crate::core::objex::geospec::{Bounds, GeoSpec, Volume, SurfaceArea};
use crate::core::objex::geospec::profile::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AuthoringShape {
    Point(Point),
    Line(Line),
    Plane(Plane),
    Sphere(Sphere),
    Box(BoxShape),
    Cylinder(Cylinder),
    Cone(Cone),
    Union {
    children: Vec<AuthoringShape>,
    },
    Difference {
        outer: Box<AuthoringShape>,
        inner: Box<AuthoringShape>,
    },
    // new
    Extrude {
        profile: Profile,
        length: f64,
        capped: bool,
    },


}


impl AuthoringShape {
    pub fn compile(&self) -> GeoSpec {
        GeoSpec {
            bounds: Bounds::from_shape(self),
            volume: self.volume(),
            surface_area: self.surface_area(),
        }
    }
}

impl SurfaceArea for AuthoringShape {
    fn surface_area(&self) -> f64 {
        match self {
            AuthoringShape::Point(_) => 0.0,
            AuthoringShape::Line(_) => 0.0,
            AuthoringShape::Plane(p) => p.surface_area(),
            AuthoringShape::Sphere(s) => s.surface_area(),
            AuthoringShape::Box(b) => b.surface_area(),
            AuthoringShape::Cylinder(c) => c.surface_area(),
            AuthoringShape::Cone(c) => c.surface_area(),
            AuthoringShape::Union { children } => {
                children.iter().map(|c| c.surface_area()).sum()
            }

            AuthoringShape::Difference { outer, inner } => {
                // Outer surface + inner exposed surface
                outer.surface_area() + inner.surface_area()
            }
            AuthoringShape::Extrude { profile, length, capped } => {
                let side = profile.perimeter() * length;
                let caps = if *capped {
                    2.0 * profile.area()
                } else {
                    0.0
                };
                side + caps
            }


        }
    }
}

impl Volume for AuthoringShape {
    fn volume(&self) -> f64 {
        match self {
            AuthoringShape::Sphere(s) => s.volume(),
            AuthoringShape::Box(b) => b.volume(),
            AuthoringShape::Cylinder(c) => c.volume(),
            AuthoringShape::Cone(c) => c.volume(),
            AuthoringShape::Union { children } => {
                children.iter().map(|c| c.volume()).sum()
            }

            AuthoringShape::Difference { outer, inner } => {
                outer.volume() - inner.volume()
            }
            AuthoringShape::Extrude { profile, length, capped } => {
                profile.area() * length
            }

            _ => 0.0,
        }
    }
}




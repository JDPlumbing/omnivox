use serde::{Serialize, Deserialize};
use crate::core::objex::matcat::materials::MatCatId;

use crate::core::objex::geospec::shapes::{Shape, Sphere, BoxShape, Cylinder};

/// Canonical blueprint for a physical object.
/// Identified by a stable UUID.


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objex {
    pub shape: Shape,
    pub material: MatCatId,
}


impl Objex {
    pub fn new(shape: Shape, material: MatCatId) -> Self {
        Self { shape, material }
    }

    pub fn sphere(material: MatCatId, radius: f64) -> Self {
        Self::new(
            Shape::Sphere(Sphere { radius }),
            material,
        )
    }

    pub fn box_shape(material: MatCatId, l: f64, w: f64, h: f64) -> Self {
        Self::new(
            Shape::Box(BoxShape { length: l, width: w, height: h }),
            material,
        )
    }

    pub fn cylinder(material: MatCatId, radius: f64, height: f64) -> Self {
        Self::new(
            Shape::Cylinder(Cylinder { radius, height }),
            material,
        )
    }
}


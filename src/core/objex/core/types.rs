use serde::{Serialize, Deserialize};
use crate::core::objex::matcat::MaterialLink;
use crate::core::objex::geospec::shapes::{Shape, Sphere, BoxShape, Cylinder};

/// A static blueprint for a physical object: geometry + material.
/// Contains *no* world identity, *no* position, *no* time.
///
/// If two Objex instances have the same Shape and the same MaterialLink
/// (i.e., same MatCatId), they represent the exact same object blueprint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objex {
    pub shape: Shape,
    pub material: MaterialLink,
}

impl Objex {
    /// Construct directly from shape + material.
    pub fn new(shape: Shape, material: MaterialLink) -> Self {
        Self { shape, material }
    }

    /// Convenience constructors (no names, no UUIDs, pure blueprint)

    pub fn sphere(material: MaterialLink, radius: f64) -> Self {
        Self::new(
            Shape::Sphere(Sphere { radius }),
            material,
        )
    }

    pub fn box_shape(material: MaterialLink, l: f64, w: f64, h: f64) -> Self {
        Self::new(
            Shape::Box(BoxShape { length: l, width: w, height: h }),
            material,
        )
    }

    pub fn cylinder(material: MaterialLink, radius: f64, height: f64) -> Self {
        Self::new(
            Shape::Cylinder(Cylinder { radius, height }),
            material,
        )
    }
}

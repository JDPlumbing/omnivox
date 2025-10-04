use uuid::Uuid;
use crate::geospec::shapes::*;
use crate::objex::{Objex, Shape, MaterialLink};

/// Builder pattern for Objex
pub struct ObjexBuilder {
    entity_id: Uuid,
    name: Option<String>,
    shape: Option<Shape>,
    material: Option<MaterialLink>,
}

impl Default for ObjexBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjexBuilder {
    pub fn new() -> Self {
        Self {
            entity_id: Uuid::new_v4(),
            name: None,
            shape: None,
            material: None,
        }
    }

    pub fn id(mut self, id: Uuid) -> Self {
        self.entity_id = id;
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape = Some(shape);
        self
    }

    pub fn material(mut self, material: MaterialLink) -> Self {
        self.material = Some(material);
        self
    }

    pub fn build(self) -> Objex {
        Objex {
            frame_id: 0, // 🔥 include this
            entity_id: self.entity_id,
            name: self.name.unwrap_or_else(|| "unnamed".into()),
            shape: self.shape.unwrap_or(Shape::Point(Point)),
            material: self.material.unwrap_or_else(MaterialLink::vacuum),
        }
    }
}

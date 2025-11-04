use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::geospec::shapes::{Point, Line, Plane, Sphere, BoxShape, Cylinder, Cone};
use serde_json::json;
use crate::uvoxid::UvoxId;
use crate::supabasic::objex::ObjectRecord;


/// Shape is an enum that can represent any geometric primitive
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
        Shape::Point(Point) // fallback if nothing is set
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


/// Broad categories used by systems (physics, corrosion, etc.)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MaterialKind {
    Metal,
    Ceramic,
    Polymer,
    Organic,
    Masonry,
    Glass,
    Liquid,
    Gas,
    Composite,
    Other,
}

/// Named materials.
/// Each one implicitly maps to a MaterialKind.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MaterialName {
    // Common ones
    Concrete,
    Steel,
    Copper,
    Aluminum,
    Wood,
    Plastic,
    Rubber,
    Glass,
    Air,
    Water,

    // Catch-all for anything not predefined
    Custom(String),
}

/// Lightweight material reference.
/// Always carries both name + kind.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialLink {
    pub id: Uuid,
    pub name: MaterialName,
    pub kind: MaterialKind,
}

impl MaterialLink {
    pub fn new(name: MaterialName) -> Self {
        let kind = match &name {
            MaterialName::Concrete => MaterialKind::Masonry,
            MaterialName::Steel => MaterialKind::Metal,
            MaterialName::Copper => MaterialKind::Metal,
            MaterialName::Aluminum => MaterialKind::Metal,
            MaterialName::Wood => MaterialKind::Organic,
            MaterialName::Plastic => MaterialKind::Polymer,
            MaterialName::Rubber => MaterialKind::Polymer,
            MaterialName::Glass => MaterialKind::Glass,
            MaterialName::Air => MaterialKind::Gas,
            MaterialName::Water => MaterialKind::Liquid,
            MaterialName::Custom(_) => MaterialKind::Other,
        };

        Self {
            id: Uuid::new_v4(),
            name,
            kind,
        }
    }

    /// Example: treat “vacuum” as just air for now
    pub fn vacuum() -> Self {
        Self::new(MaterialName::Air)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objex {
    pub frame_id: i64,
    pub entity_id: Uuid,
    pub property_id: Option<Uuid>,
    pub uvoxid: UvoxId,
    pub name: String,
    pub shape: Shape,
    pub material: MaterialLink,
    pub metadata: Option<HashMap<String, String>>,
}



impl Objex {
    pub fn new_sphere(frame_id: i64, property_id: Option<Uuid>, material: MaterialLink, radius: f64) -> Self {
        Self {
            frame_id,
            property_id,
            uvoxid: UvoxId::new(frame_id, 0, 0, 0), // TODO: real coords later
            entity_id: Uuid::new_v4(),
            name: format!("{:?} Sphere", material),
            shape: Shape::Sphere(Sphere { radius }),
            material,
            metadata: None,
        }
    }

    pub fn new_box(frame_id: i64, property_id: Option<Uuid>, material: MaterialLink, length: f64, width: f64, height: f64) -> Self {
        Self {
            frame_id,
            property_id,
            uvoxid: UvoxId::new(frame_id, 0, 0, 0), // TODO: real coords later
            entity_id: Uuid::new_v4(),
            name: format!("{:?} Box", material),
            shape: Shape::Box(BoxShape { length, width, height }),
            material,
            metadata: None,
        }
    }

    pub fn new_cylinder(frame_id: i64, property_id: Option<Uuid>, material: MaterialLink, radius: f64, height: f64) -> Self {
        Self {
            frame_id,
            property_id,
            uvoxid: UvoxId::new(frame_id, 0, 0, 0), // TODO: real coords later
            entity_id: Uuid::new_v4(),
            name: format!("{:?} Cylinder", material),
            shape: Shape::Cylinder(Cylinder { radius, height }),
            material,
            metadata: None,
        }
    }

    pub fn new(frame_id: i64, property_id: Option<Uuid>, name: impl Into<String>, shape: Shape, material: MaterialLink) -> Self {
        Self {
            frame_id,
            property_id,
            uvoxid: UvoxId::new(frame_id, 0, 0, 0), // TODO: real coords later
            entity_id: Uuid::new_v4(),
            name: name.into(),
            shape,
            material,
            metadata: None,
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut map = self.metadata.unwrap_or_default();
        map.insert(key.into(), value.into());
        self.metadata = Some(map);
        self
    }
}



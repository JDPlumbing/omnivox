use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::geospec::shapes::{Point, Line, Plane, Sphere, BoxShape, Cylinder, Cone};
use serde_json::{json, Value};
use crate::uvoxid::UvoxId;
use crate::supabasic::objex::ObjectRecord;
use crate::matcat;
use crate::geospec::traits::{Dimensions, Volume, SurfaceArea};

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
    Plasma,
    Geologic,
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
    Plasma,
    Soil,

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
    pub matcat_id: Option<matcat::MatCatId>,
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
            MaterialName::Plasma => MaterialKind::Plasma,
            MaterialName::Soil => MaterialKind::Geologic,

            MaterialName::Custom(_) => MaterialKind::Other,
        };

        // Attempt to auto-link to matcat if a standard material
        let matcat_id = matcat::MatCatId::from_name(&name);

        Self {
            id: Uuid::new_v4(),
            name,
            kind,
            matcat_id,
        }
    }

    pub fn vacuum() -> Self {
        Self::new(MaterialName::Air)
    }

    pub fn props(&self) -> Option<matcat::MatProps> {
        self.matcat_id.map(|id| matcat::props_for(&id))
    }

        /// Return a formatted human-readable description of the material
    pub fn describe(&self) -> String {
        let name = format!("{:?}", self.name);
        let kind = format!("{:?}", self.kind);

        let matcat_str = if let Some(id) = &self.matcat_id {
            let label = id.name();
            let props = id.props();
            if let Some(p) = props {
                format!(
                    "{} ({})\n  density = {:.2} kg/m³\n  elastic_modulus = {:.2e} Pa\n  hardness = {:.2}\n  melting_point = {:.1}°C",
                    label,
                    kind,
                    p.density,
                    p.elastic_modulus,
                    p.hardness,
                    p.melting_point
                )
            } else {
                format!("{} ({}) [no props found]", label, kind)
            }
        } else {
            format!("{} ({}) [no MatCat ID linked]", name, kind)
        };

        matcat_str
    }

        /// Return a structured JSON description of the material
    pub fn describe_json(&self) -> Value {
        let matcat_info = if let Some(id) = &self.matcat_id {
            let label = id.name();
            let props = id.props();
            match props {
                Some(p) => json!({
                    "name": label,
                    "category": id.category,
                    "variant": id.variant,
                    "grade": id.grade,
                    "properties": {
                        "density": p.density,
                        "elastic_modulus": p.elastic_modulus,
                        "tensile_strength": p.tensile_strength,
                        "compressive_strength": p.compressive_strength,
                        "hardness": p.hardness,
                        "fracture_toughness": p.fracture_toughness,
                        "thermal_conductivity": p.thermal_conductivity,
                        "thermal_expansion": p.thermal_expansion,
                        "melting_point": p.melting_point,
                        "corrosion_resistance": p.corrosion_resistance,
                        "flammability": p.flammability,
                        "electrical_conductivity": p.electrical_conductivity,
                        "magnetic_permeability": p.magnetic_permeability,
                    }
                }),
                None => json!({
                    "name": label,
                    "category": id.category,
                    "variant": id.variant,
                    "grade": id.grade,
                    "properties": null
                }),
            }
        } else {
            json!({
                "name": format!("{:?}", self.name),
                "kind": format!("{:?}", self.kind),
                "matcat_id": null,
                "properties": null
            })
        };

        matcat_info
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

impl Dimensions for Shape {
    fn as_json(&self) -> serde_json::Value {
        match self {
            Shape::Point(p) => p.as_json(),
            Shape::Line(l) => l.as_json(),
            Shape::Plane(pl) => pl.as_json(),
            Shape::Sphere(s) => s.as_json(),
            Shape::Box(b) => b.as_json(),
            Shape::Cylinder(c) => c.as_json(),
            Shape::Cone(cn) => cn.as_json(),
        }
    }
}

impl Volume for Shape {
    fn volume(&self) -> f64 {
        match self {
            Shape::Box(b) => b.volume(),
            Shape::Sphere(s) => s.volume(),
            Shape::Cylinder(c) => c.volume(),
            Shape::Cone(cn) => cn.volume(),
            _ => 0.0,
        }
    }
}

impl SurfaceArea for Shape {
    fn surface_area(&self) -> f64 {
        match self {
            Shape::Box(b) => b.surface_area(),
            Shape::Sphere(s) => s.surface_area(),
            Shape::Cylinder(c) => c.surface_area(),
            Shape::Cone(cn) => cn.surface_area(),
            Shape::Plane(p) => p.surface_area(),
            _ => 0.0,
        }
    }
}
